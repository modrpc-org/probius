#[cfg(feature = "enabled")]
use core::{
    cell::Cell,
    future::{Future, poll_fn},
    pin::{Pin, pin},
    ptr::NonNull,
};

use crate::{Source, SourceId};

#[cfg(feature = "enabled")]
use crate::Probius;

#[cfg(feature = "enabled")]
thread_local! {
    static CURRENT_COMPONENT: Cell<Option<NonNull<()>>> = Cell::new(None);
}

pub struct Component {
    source: Source,
}

impl Component {
    #[cfg(feature = "enabled")]
    pub(crate) fn new(probius: Probius, name: &str, is_recurring: bool) -> Self {
        Self {
            source: Source::new(probius, name, is_recurring),
        }
    }

    pub fn id(&self) -> SourceId {
        self.source.id()
    }

    #[cfg(feature = "enabled")]
    #[inline]
    pub fn enter<R>(&mut self, f: impl FnOnce() -> R) -> R {
        // self cannot move for the duration of this method.
        let self_pin = Pin::new(self);

        let parent = CURRENT_COMPONENT.replace(Some(NonNull::from(&*self_pin).cast()));
        let result = f();
        CURRENT_COMPONENT.set(parent);
        result
    }

    #[cfg(feature = "enabled")]
    #[inline]
    pub async fn enter_async<F: Future>(&mut self, f: F) -> F::Output {
        let mut f = pin!(f);
        poll_fn(|cx| {
            self.enter(|| {
                f.as_mut().poll(cx)
            })
        })
        .await
    }

    #[cfg(not(feature = "enabled"))]
    #[inline]
    pub fn enter<R>(&mut self, f: impl FnOnce() -> R) -> R {
        f()
    }

    #[cfg(not(feature = "enabled"))]
    #[inline]
    pub async fn enter_async<F: Future>(&mut self, f: F) -> F::Output {
        f.await
    }
}

#[cfg(feature = "enabled")]
#[inline]
pub(crate) fn with_current<R>(f: impl FnOnce(Option<&Component>) -> R) -> R {
    let component = CURRENT_COMPONENT.get()
        .map(|ptr| unsafe { ptr.cast().as_ref() });
    f(component)
}

