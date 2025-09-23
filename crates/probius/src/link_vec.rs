use core::{
    cell::{Cell, OnceCell, UnsafeCell},
    hash::Hash,
    marker::PhantomData,
    mem::MaybeUninit,
    ops::Deref,
    ptr::NonNull,
};

#[derive(Debug)]
pub struct LinkVecPtr<T> {
    ptr: NonNull<T>,
}

impl<T> Copy for LinkVecPtr<T> { }

impl<T> Clone for LinkVecPtr<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T> PartialEq for LinkVecPtr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for LinkVecPtr<T> { }

impl<T> Hash for LinkVecPtr<T> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
        where H: core::hash::Hasher
    {
        self.ptr.hash(state);
    }
}

impl<T> Deref for LinkVecPtr<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

/// LinkVec is a niche arena datastructure that only allows items to be inserted and immutably
/// referenced. Inserted items will be dropped only when the containing LinkVec is dropped.
///
/// Insertion returns a LinkVecPtr which can be used to immutably reference the inserted item.
///
/// A LinkVec can only be constructed via the `LinkVec::leak()` constructor. As the name suggests,
/// the returned LinkVec will leak memory unless explicitly released via the `unleak` method.
/// `LinkVec::unleak` is an unsafe operation - it must only be called once and the caller must
/// guarantee that no LinkVecPtr from the LinkVec will be dereferenced afterwards.
pub struct LinkVec<T, const N: usize = 8> {
    head: OnceCell<NonNull<LinkVecNode<T, N>>>,
    tail: Cell<Option<NonNull<LinkVecNode<T, N>>>>,
    len: Cell<usize>,
}

pub struct LinkVecNode<T, const N: usize = 8> {
    next: OnceCell<NonNull<LinkVecNode<T, N>>>,
    previous: Option<NonNull<LinkVecNode<T, N>>>,
    len: Cell<usize>,
    slots: [SlotData<T>; N],
}

struct SlotData<T> {
    data: UnsafeCell<MaybeUninit<T>>,
}

impl<T, const N: usize> LinkVec<T, N> {
    /// Construct an empty LinkVec. No allocations will be incurred until the first item is
    /// inserted.
    ///
    /// Once an item is inserted, this LinkVec will leak memory unless `LinkVec::unleak` is called
    /// on it.
    pub fn leak() -> Self {
        Self {
            head: OnceCell::new(),
            tail: Cell::new(None),
            len: Cell::new(0),
        }
    }

    /// Get the current number of items in the LinkVec.
    pub fn len(&self) -> usize {
        self.len.get()
    }

    /// Drop all contained items and release the backing memory.
    ///
    /// Dereferencing any LinkVecPtr from this LinkVec afterwards will result in undefined
    /// behavior.
    pub unsafe fn unleak(&mut self) {
        // Drop items/nodes from newest to oldest.
        let mut tail = self.tail.get();
        while let Some(next) = tail {
            let next_ref = unsafe { next.as_ref() };
            tail = next_ref.previous;

            // A node of length 0 is not possible, but the code to handle the impossible case is
            // more concise than the proof that it's impossible.
            if next_ref.len.get() > 0 {
                let mut index = next_ref.len.get() - 1;
                while index > 0 {
                    unsafe {
                        MaybeUninit::assume_init_drop(&mut *next_ref.slots[index].data.get());
                    }
                    index -= 1;
                }
            }

            unsafe {
                let _ = Box::from_raw(next.as_ptr());
            }
        }
    }

    pub fn push(&self, item: T) -> LinkVecPtr<T> {
        let mut tail = self.tail.get().unwrap_or_else(|| {
            let head = NonNull::new(
                Box::into_raw(Box::new(LinkVecNode {
                    next: OnceCell::new(),
                    previous: None,
                    len: Cell::new(0),
                    slots: [const { SlotData { data: UnsafeCell::new(MaybeUninit::uninit()) } }; N],
                }))
            )
            .expect("LinkVec NonNull head");
            self.head.set(head).expect("LinkVec set head");
            self.tail.set(Some(head));
            head
        });

        let tail_ref = unsafe { tail.as_ref() };
        if tail_ref.len.get() == N {
            // Tail node is full, create the next node.
            let new_tail = NonNull::new(
                Box::into_raw(Box::new(LinkVecNode {
                    next: OnceCell::new(),
                    previous: Some(tail),
                    len: Cell::new(0),
                    slots: [const { SlotData { data: UnsafeCell::new(MaybeUninit::uninit()) } }; N],
                }))
            )
            .expect("LinkVec NonNull new tail");
            tail_ref.next.set(new_tail).expect("LinkVec set tail.next");
            self.tail.set(Some(new_tail));
            tail = new_tail;
        }

        let tail_ref = unsafe { tail.as_ref() };
        // Allocate the next free slot
        let index = tail_ref.len.get();
        tail_ref.len.set(index + 1);
        // Write the slot
        let slot = &unsafe { tail.as_ref() }.slots[index];
        unsafe { &mut *slot.data.get() }.write(item);

        self.len.set(self.len.get() + 1);

        LinkVecPtr {
            ptr: tail_ref.slot_data_ptr(index),
        }
    }

    pub fn iter<'a>(&'a self) -> LinkVecIter<'a, T, N> {
        LinkVecIter {
            head: self.head.get().copied(),
            len: self.len.get(),
            node_index: 0,
            _lifetime: PhantomData,
        }
    }
}

impl<T, const N: usize> LinkVecNode<T, N> {
    fn slot_data_ptr(&self, index: usize) -> NonNull<T> {
        let slot = &self.slots[index];
        let ptr = unsafe { &mut *slot.data.get() }.as_mut_ptr();
        unsafe { NonNull::new_unchecked(ptr) }
    }
}

pub struct LinkVecIter<'a, T, const N: usize> {
    head: Option<NonNull<LinkVecNode<T, N>>>,
    len: usize,
    node_index: usize,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a, T: std::fmt::Debug + 'a, const N: usize> Iterator for LinkVecIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(mut head) = self.head else {
            return None;
        };

        let mut head_ref = unsafe { head.as_ref() };
        if self.node_index == head_ref.len.get() {
            if let Some(next) = head_ref.next.get().copied() {
                self.head = Some(next);
                self.node_index = 0;
                head = next;
                head_ref = unsafe { head.as_ref() };
            } else {
                return None;
            }
        }

        let index = self.node_index;
        self.node_index += 1;
        self.len -= 1;
        let slot_data_ptr = head_ref.slot_data_ptr(index);
        Some(unsafe { slot_data_ptr.as_ref() })
    }
}

impl<'a, T: std::fmt::Debug + 'a, const N: usize> ExactSizeIterator for LinkVecIter<'a, T, N> {
    fn len(&self) -> usize { self.len }
}

impl<T, const N: usize> Clone for LinkVecIter<'_, T, N> {
    fn clone(&self) -> Self {
        Self {
            head: self.head,
            len: self.len,
            node_index: self.node_index,
            _lifetime: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_link_vec_iter() {
        let mut l = LinkVec::<u32, 2>::leak();
        let n1 = l.push(1);
        let n2 = l.push(2);
        let n3 = l.push(3);

        assert_eq!(*n1, 1);
        assert_eq!(*n2, 2);
        assert_eq!(*n3, 3);
        assert_eq!(l.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);

        unsafe { l.unleak(); }
    }
}
