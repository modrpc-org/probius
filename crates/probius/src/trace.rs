use core::{
    cell::{Cell, OnceCell, RefCell, UnsafeCell},
    future::Future,
    ptr::NonNull,
    sync::{
        atomic::{AtomicU64, Ordering},
    },
};
use std::rc::Rc;

use probius_mproto::{GlobalSourceId, MetricAggregate, SourceId};
use spin::Mutex;

use crate::{
    component::{self, Component},
    encoding::ProbiusWriter,
    link_vec::{LinkVec, LinkVecPtr},
    void_sink,
};

static NEXT_SOURCE_ID: AtomicU64 = AtomicU64::new(0);
thread_local! {
    static NEXT_EVENT_SEQ: Cell<u16> = Cell::new(0);
    static PROBIUS: OnceCell<Probius> = OnceCell::new();
}

static APP_CONFIG: Mutex<Option<AppConfig>> = Mutex::new(None);

struct AppConfig {
    buffer_headroom: usize,
    buffer_pool: bab::HeapBufferPool,
}

pub fn init(buffer_headroom: usize, buffer_pool: bab::HeapBufferPool) {
    // bab requires that there are at least `num_threads * 3 / 2` batches to avoid any single
    // thread getting starved. In Probius's case, there will be at most 2 threads - the
    // publisher and the flusher (which may or may not run on the same thread).
    //
    // So we need at least 3 batches, but 4 divides better since people tend to supply large
    // even numbers for buffer counts.
    //let batch_count = 4;
    //let buffers_per_batch = (min_buffer_count + batch_count - 1) / batch_count;
    //let buffer_pool = bab::HeapBufferPool::new(buffer_size, batch_count, buffers_per_batch);

    let mut app_config = APP_CONFIG.lock();
    if app_config.is_some() {
        drop(app_config);
        panic!("probius::init called twice");
    } else {
        *app_config = Some(AppConfig { buffer_headroom, buffer_pool });
    }
}

fn with_probius<R>(f: impl FnOnce(&Probius) -> R) -> R {
    try_with_probius(move |probius| {
        f(probius)
    })
}

fn try_with_probius<R>(f: impl FnOnce(&Probius) -> R) -> R {
    PROBIUS.with(move |probius| {
        let probius = probius.get_or_init(move || {
            let mut maybe_app_config = APP_CONFIG.lock();
            let app_config = maybe_app_config.get_or_insert_with(|| {
                // Default to the void sink if none was setup by the application.
                let buffer_pool = bab::HeapBufferPool::new(8192, 16, 16);
                void_sink::spawn_void_sink_flusher(buffer_pool.clone());
                AppConfig { buffer_headroom: 0, buffer_pool }
            });

            Probius::new(app_config.buffer_headroom, app_config.buffer_pool.clone())
        });

        f(probius)
    })
}

pub fn flush() -> impl Iterator<Item = bab::BufferPtr> {
    with_probius(|probius| probius.inner.flush())
}

pub fn enter_component<R>(name: &'static str, f: impl FnOnce() -> R) -> R {
    try_with_probius(|probius| {
        Component::new(probius.clone(), name, false).enter(f)
    })
}

pub async fn enter_component_async<F: Future>(name: &'static str, f: F) -> F::Output {
    let probius = try_with_probius(|probius| probius.clone());

    Component::new(probius, name, false).enter_async(f).await
}

pub fn enter_component_ephemeral<R>(name: &'static str, f: impl FnOnce() -> R) -> R {
    try_with_probius(|probius| {
        Component::new(probius.clone(), name, false).enter(f)
    })
}

pub async fn enter_component_ephemeral_async<F: Future>(name: &'static str, f: F) -> F::Output {
    let probius = try_with_probius(|probius| probius.clone());

    Component::new(probius, name, false).enter_async(f).await
}

pub fn new_trace_source(name: &'static str) -> TraceSource {
    try_with_probius(|probius| {
        TraceSource::new(probius.clone(), name, true)
    })
}

pub fn new_trace_source_ephemeral(name: &'static str) -> TraceSource {
    try_with_probius(|probius| {
        TraceSource::new(probius.clone(), name, false)
    })
}

#[derive(Clone)]
pub struct Probius {
    inner: Rc<ProbiusWriter>,
}

impl Probius {
    fn new(
        buffer_headroom: usize,
        buffer_pool: bab::HeapBufferPool,
    ) -> Self {
        Self {
            inner: Rc::new(ProbiusWriter::new(buffer_headroom, buffer_pool.clone())),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventSeq(pub u16);

/// An event is identified mainly by its source and its timestamp. An event's seq is not unique and
/// is only used to differentiate multiple events from the same source occurring at the same
/// timestamp.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventId {
    pub source: SourceId,
    pub timestamp_nanos: u64,
    pub seq: EventSeq,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TraceOp {
    CreateSource { source: SourceId },
    DeleteSource { source: SourceId },
    Call { source: SourceId },
    PushScope,
    PopScope,
    BranchStart,
    BranchEnd,
    Label { label: *const str },
    Tag,
    Metric { name: &'static str, value: i64 },

    LocalChannelSend { channel: SourceId, version: u64 },
    LocalChannelReceive { channel: SourceId, version: u64 },
    LocalChannelTransferFrom {
        from: SourceId,
        from_version: u64,
        to: SourceId,
        to_version: u64,
    },

    GlobalChannelSend { channel: GlobalSourceId, version: u64 },
    GlobalChannelReceive { channel: GlobalSourceId, version: u64 },
    GlobalChannelTransferFrom {
        from: GlobalSourceId,
        from_version: u64,
        to: GlobalSourceId,
        to_version: u64,
    },
}

impl TraceOp {
    #[inline]
    fn as_op_aggregate(&self) -> TraceOpAggregate {
        match self {
            TraceOp::CreateSource { .. } => TraceOpAggregate::CreateSource,
            TraceOp::DeleteSource { .. } => TraceOpAggregate::DeleteSource,
            TraceOp::Call { source } => TraceOpAggregate::Call { source: *source },
            TraceOp::PushScope => TraceOpAggregate::PushScope,
            TraceOp::PopScope => TraceOpAggregate::PopScope,
            TraceOp::BranchStart => TraceOpAggregate::BranchStart,
            TraceOp::BranchEnd => TraceOpAggregate::BranchEnd,
            TraceOp::Label { label } => TraceOpAggregate::Label { label: *label }, 
            TraceOp::Tag { .. } => TraceOpAggregate::Tag,
            TraceOp::Metric { name, .. } => TraceOpAggregate::Metric { name },

            TraceOp::LocalChannelSend { channel, .. } =>
                TraceOpAggregate::LocalChannelSend { channel: *channel },
            TraceOp::LocalChannelReceive { channel, .. } =>
                TraceOpAggregate::LocalChannelReceive { channel: *channel },
            TraceOp::LocalChannelTransferFrom { from, to, .. } =>
                TraceOpAggregate::LocalChannelTransferFrom { from: *from, to: *to },

            TraceOp::GlobalChannelSend { channel, .. } =>
                TraceOpAggregate::GlobalChannelSend { channel: *channel },
            TraceOp::GlobalChannelReceive { channel, .. } =>
                TraceOpAggregate::GlobalChannelReceive { channel: *channel },
            TraceOp::GlobalChannelTransferFrom { from, to, .. } =>
                TraceOpAggregate::GlobalChannelTransferFrom {
                    from: *from,
                    to: *to,
                },
        }
    }
}

pub struct Source {
    probius: Probius,

    id: SourceId,
    #[cfg(not(target_arch = "wasm32"))]
    create_time: std::time::Instant,
}

impl Source {
    pub(crate) fn new(probius: Probius, name: &'static str, is_recurring: bool) -> Self {
        let source = Self {
            probius: probius.clone(),

            id: SourceId { source: NEXT_SOURCE_ID.fetch_add(1, Ordering::Relaxed) },
            #[cfg(not(target_arch = "wasm32"))]
            create_time: std::time::Instant::now(),
        };

        component::with_current(|parent| {
            probius.inner.create_source(
                source.next_event_id(),
                name,
                parent.map(|p| p.id()),
                is_recurring,
            );
        });

        source
    }

    pub fn id(&self) -> SourceId { self.id }

    #[cfg(not(target_arch = "wasm32"))]
    fn now_nanos(&self) -> u64 {
        self.create_time.elapsed().as_nanos() as u64
    }

    #[cfg(target_arch = "wasm32")]
    fn now_nanos(&self) -> u64 {
        // TODO
        0
    }

    fn next_event_id(&self) -> probius_mproto::EventId {
        let seq = NEXT_EVENT_SEQ.get();
        NEXT_EVENT_SEQ.set(seq.wrapping_add(1));
        let timestamp_nanos = self.now_nanos();
        probius_mproto::EventId {
            source: self.id,
            timestamp_nanos,
            seq: probius_mproto::EventSeq { seq },
        }
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        self.probius.inner.delete_source(self.next_event_id());
    }
}

thread_local! {
    static TRACE_STACK: Cell<Option<NonNull<()>>> = Cell::new(None);
}

pub struct TraceSource {
    source: Source,
    trace_aggregator: TraceAggregator,
}

impl TraceSource {
    fn new(probius: Probius, name: &'static str, is_recurring: bool) -> Self {
        Self {
            source: Source::new(probius, name, is_recurring),
            trace_aggregator: TraceAggregator::new(),
        }
    }

    #[inline]
    pub fn trace<R>(&self, f: impl FnOnce() -> R) -> R {
        let trace = Trace {
            is_detailed_trace: false,
            start_nanos: self.source.now_nanos(),
            trace_source: &self,
            aggregate_cursor: TraceAggregateCursor::start_cursor(),
            encode_cursor: Cell::new(0),
            encode_buf: UnsafeCell::new([0; 512]),
        };
        let parent = TRACE_STACK.replace(Some(NonNull::from(&trace).cast()));

        let result = f();

        TRACE_STACK.set(parent);
        drop(trace);

        result
    }

    #[inline]
    pub async fn trace_future<R>(&self, f: impl core::future::Future<Output = R>) -> R {
        let trace = Trace {
            is_detailed_trace: false,
            start_nanos: self.source.now_nanos(),
            trace_source: self,
            aggregate_cursor: TraceAggregateCursor::start_cursor(),
            encode_cursor: Cell::new(0),
            encode_buf: UnsafeCell::new([0; 512]),
        };

        let mut f = core::pin::pin!(f);
        let result = core::future::poll_fn(|cx| {
            let parent = TRACE_STACK.replace(Some(NonNull::from(&trace).cast()));
            let result = f.as_mut().poll(cx);
            TRACE_STACK.set(parent);
            result
        })
        .await;

        drop(trace);

        result
    }

    pub fn flush_aggregate_full(&self) {
        self.trace_aggregator.flush_full(&self.source);
    }
}

#[inline]
fn with_current_trace(f: impl FnOnce(&Trace)) {
    if let Some(trace_ptr) = TRACE_STACK.get() {
        let trace: &Trace = unsafe { trace_ptr.cast().as_ref() };
        f(trace);
    }
}

/*pub fn trace_create_source(name: &'static str) -> Source {
    let source = Source::new(name);
    with_current_trace(|trace| {
        trace.push_op(TraceOp::CreateSource { source: source.id });
    });
    source
}*/

#[inline]
pub fn trace_metric(name: &'static str, value: i64) {
    with_current_trace(|trace| trace.metric(name, value));
}

#[inline]
pub fn trace_label(label: &'static str) {
    with_current_trace(|trace| trace.label(label));
}

#[inline]
pub fn trace_branch<R>(f: impl FnOnce() -> R) -> R {
    if let Some(trace_ptr) = TRACE_STACK.get() {
        let trace: &Trace = unsafe { trace_ptr.cast().as_ref() };

        trace.branch_start();
        let result = f();
        trace.branch_end();
        result
    } else {
        f()
    }
}

#[inline]
pub fn trace_branch_start() {
    with_current_trace(|trace| trace.branch_start());
}

#[inline]
pub fn trace_branch_end() {
    with_current_trace(|trace| trace.branch_end());
}

pub struct Trace<'a> {
    is_detailed_trace: bool,
    start_nanos: u64,
    trace_source: &'a TraceSource,
    aggregate_cursor: TraceAggregateCursor,
    encode_cursor: Cell<usize>,
    encode_buf: UnsafeCell<[u8; 512]>,
}

impl Trace<'_> {
    /*fn create_source(&self, probius: Probius, name: &'static str, is_recurring: bool) -> Source {
        let source = Source::new(probius, name, is_recurring);
        self.push_op(TraceOp::CreateSource { source: source.id });
        source
    }*/

    #[inline]
    fn metric(&self, name: &'static str, value: i64) {
        self.push_op(TraceOp::Metric { name, value });
    }

    #[inline]
    fn label(&self, label: &'static str) {
        self.push_op(TraceOp::Label { label });
    }

    #[inline]
    fn branch_start(&self) {
        self.push_op(TraceOp::BranchStart);
    }

    #[inline]
    fn branch_end(&self) {
        self.push_op(TraceOp::BranchEnd);
    }

    #[inline]
    fn push_op(&self, op: TraceOp) {
        let op_node_index = self.trace_source.trace_aggregator.ingest(&self.aggregate_cursor, &op);

        if self.is_detailed_trace {
            if let Err(()) = self.try_write_op(op_node_index, op) {
                // TODO mark trace buffer as invalid
            }
        }
    }

    fn try_write_op(&self, op_node_index: u16, op: TraceOp) -> Result<(), ()> {
        self.try_write_mproto(op_node_index)?;
        match op {
            TraceOp::CreateSource { source } => {
                self.try_write_mproto(source)?;
            }
            TraceOp::DeleteSource { source } => {
                self.try_write_mproto(source)?;
            }
            TraceOp::Call { .. } => {
                // TODO can we identify the specific child trace here?
            }
            TraceOp::PushScope => { }
            TraceOp::PopScope => { }
            TraceOp::BranchStart => { }
            TraceOp::BranchEnd => { }
            TraceOp::Label { .. } => { }
            TraceOp::Tag => {
                // TODO
            }
            TraceOp::Metric { value, .. } => {
                self.try_write_mproto(value)?;
            }

            TraceOp::LocalChannelSend { .. } => {
            }
            TraceOp::LocalChannelReceive { .. } => {
            }
            TraceOp::LocalChannelTransferFrom { .. } => {
            }

            TraceOp::GlobalChannelSend { .. } => {
            }
            TraceOp::GlobalChannelReceive { .. } => {
            }
            TraceOp::GlobalChannelTransferFrom { .. } => {
            }
        }

        Ok(())
    }

    #[inline]
    fn try_write_mproto(&self, value: impl mproto::Encode) -> Result<(), ()> {
        let encoded_len = mproto::encoded_len(&value);
        let write_buf = self.try_write(encoded_len)?;
        mproto::encode_value(value, write_buf);
        Ok(())
    }

    #[inline]
    fn try_write(&self, len: usize) -> Result<&mut [u8], ()> {
        let start = self.encode_cursor.get();
        if start + len > 512 {
            return Err(());
        }
        self.encode_cursor.set(start + len);

        let slice_ptr = self.encode_buf.get() as *mut u8;
        Ok(unsafe { core::slice::from_raw_parts_mut(slice_ptr.add(start), len) })
    }
}

impl Drop for Trace<'_> {
    fn drop(&mut self) {
        if self.is_detailed_trace {
            let encode_buf = unsafe { &*self.encode_buf.get() };
            self.trace_source.source.probius.inner.trace(
                self.trace_source.source.next_event_id(),
                self.start_nanos,
                &encode_buf[..self.encode_cursor.get()],
            );
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TraceOpAggregate {
    CreateSource,
    DeleteSource,
    Call { source: SourceId },
    PushScope,
    PopScope,
    BranchStart,
    BranchEnd,
    Label { label: *const str },
    Tag,
    Metric { name: &'static str },

    LocalChannelSend { channel: SourceId },
    LocalChannelReceive { channel: SourceId },
    LocalChannelTransferFrom {
        from: SourceId,
        to: SourceId,
    },

    GlobalChannelSend { channel: GlobalSourceId },
    GlobalChannelReceive { channel: GlobalSourceId },
    GlobalChannelTransferFrom {
        from: GlobalSourceId,
        to: GlobalSourceId,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum TraceAggregateNodeData {
    CreateSource,
    DeleteSource,
    Call { source: SourceId },
    PushScope,
    PopScope,
    BranchStart {
        branch_end: TraceAggregateNodePtr,
    },
    BranchEnd {
        parent_branch_end: Option<TraceAggregateNodePtr>,
    },
    Label { label: &'static str },
    Tag,
    Metric {
        name: &'static str,
        index: u16,
    },

    LocalChannelSend { channel: SourceId },
    LocalChannelReceive { channel: SourceId },
    LocalChannelTransferFrom {
        from: SourceId,
        to: SourceId,
    },

    GlobalChannelSend { channel: GlobalSourceId },
    GlobalChannelReceive { channel: GlobalSourceId },
    GlobalChannelTransferFrom {
        from: GlobalSourceId,
        to: GlobalSourceId,
    },
}

impl TraceAggregateNodeData {
    #[inline]
    fn as_op_aggregate(&self) -> TraceOpAggregate {
        match self {
            TraceAggregateNodeData::CreateSource => TraceOpAggregate::CreateSource,
            TraceAggregateNodeData::DeleteSource => TraceOpAggregate::DeleteSource,
            TraceAggregateNodeData::Call { source } => TraceOpAggregate::Call { source: *source },
            TraceAggregateNodeData::PushScope => TraceOpAggregate::PushScope,
            TraceAggregateNodeData::PopScope => TraceOpAggregate::PopScope,
            TraceAggregateNodeData::BranchStart { .. } => TraceOpAggregate::BranchStart,
            TraceAggregateNodeData::BranchEnd { .. } => TraceOpAggregate::BranchEnd,
            TraceAggregateNodeData::Label { label } => TraceOpAggregate::Label { label: *label }, 
            TraceAggregateNodeData::Tag => TraceOpAggregate::Tag,
            TraceAggregateNodeData::Metric { name, .. } => {
                TraceOpAggregate::Metric { name }
            }

            TraceAggregateNodeData::LocalChannelSend { channel } =>
                TraceOpAggregate::LocalChannelSend { channel: *channel },
            TraceAggregateNodeData::LocalChannelReceive { channel } =>
                TraceOpAggregate::LocalChannelReceive { channel: *channel },
            TraceAggregateNodeData::LocalChannelTransferFrom { from, to } =>
                TraceOpAggregate::LocalChannelTransferFrom { from: *from, to: *to },

            TraceAggregateNodeData::GlobalChannelSend { channel } =>
                TraceOpAggregate::GlobalChannelSend { channel: *channel },
            TraceAggregateNodeData::GlobalChannelReceive { channel } =>
                TraceOpAggregate::GlobalChannelReceive { channel: *channel },
            TraceAggregateNodeData::GlobalChannelTransferFrom { from, to } =>
                TraceOpAggregate::GlobalChannelTransferFrom {
                    from: *from,
                    to: *to,
                },
        }
    }

    fn as_mproto(&self)
        -> impl mproto::Encode + mproto::Compatible<probius_mproto::TraceOpAggregate>
    {
        match *self {
            TraceAggregateNodeData::CreateSource =>
                probius_mproto::TraceOpAggregate::CreateSource,
            TraceAggregateNodeData::DeleteSource =>
                probius_mproto::TraceOpAggregate::DeleteSource,
            TraceAggregateNodeData::Call { source } =>
                probius_mproto::TraceOpAggregate::Call { source },
            TraceAggregateNodeData::PushScope => probius_mproto::TraceOpAggregate::PushScope,
            TraceAggregateNodeData::PopScope => probius_mproto::TraceOpAggregate::PopScope,
            TraceAggregateNodeData::BranchStart { branch_end } =>
                probius_mproto::TraceOpAggregate::BranchStart {
                    branch_end: branch_end.index,
                },
            TraceAggregateNodeData::BranchEnd { parent_branch_end } =>
                probius_mproto::TraceOpAggregate::BranchEnd {
                    parent_branch_end: parent_branch_end.map(|n| n.index).unwrap_or(u16::MAX),
                },
            TraceAggregateNodeData::Label { label } =>
                probius_mproto::TraceOpAggregate::Label { label: label.into() },
            TraceAggregateNodeData::Tag => probius_mproto::TraceOpAggregate::Tag,
            TraceAggregateNodeData::Metric { name, index } =>
                probius_mproto::TraceOpAggregate::Metric {
                    name: name.into(),
                    index,
                },

            TraceAggregateNodeData::LocalChannelSend { channel } =>
                probius_mproto::TraceOpAggregate::ChannelSend { channel },
            TraceAggregateNodeData::LocalChannelReceive { channel } =>
                probius_mproto::TraceOpAggregate::ChannelReceive { channel },
            TraceAggregateNodeData::LocalChannelTransferFrom { from, to } =>
                probius_mproto::TraceOpAggregate::ChannelTransfer { from, to },

            TraceAggregateNodeData::GlobalChannelSend { channel } =>
                probius_mproto::TraceOpAggregate::GlobalChannelSend { channel },
            TraceAggregateNodeData::GlobalChannelReceive { channel } =>
                probius_mproto::TraceOpAggregate::GlobalChannelReceive { channel },
            TraceAggregateNodeData::GlobalChannelTransferFrom { from, to } =>
                probius_mproto::TraceOpAggregate::GlobalChannelTransfer { from, to },
        }
    }
}

#[derive(Debug)]
pub struct TraceAggregateNode {
    op: TraceAggregateNodeData,
    branch_sibling: OnceCell<TraceAggregateBranch>,
    next: OnceCell<TraceAggregateNodePtr>,
    index: u16,
}

#[derive(Debug)]
pub struct TraceAggregateBranch {
    next: TraceAggregateNodePtr,
}

pub struct TraceAggregateCursor {
    node: Cell<Option<TraceAggregateNodePtr>>,
    branch_end: Cell<Option<TraceAggregateNodePtr>>,
}

impl TraceAggregateCursor {
    #[inline]
    pub fn start_cursor() -> Self {
        Self {
            node: Cell::new(None),
            branch_end: Cell::new(None),
        }
    }
}

type TraceAggregateNodePtr = LinkVecPtr<TraceAggregateNode>;

/// Aggregator for traces from a single TraceSource
pub struct TraceAggregator {
    start_node: OnceCell<TraceAggregateNodePtr>,
    metrics: RefCell<Vec<MetricAggregate>>,
    nodes: LinkVec<TraceAggregateNode>,
}

impl TraceAggregator {
    fn new() -> Self {
        Self {
            start_node: OnceCell::new(),
            metrics: RefCell::new(Vec::new()),
            nodes: LinkVec::leak(),
        }
    }

    #[inline]
    fn ingest(&self, cursor: &TraceAggregateCursor, op: &TraceOp) -> u16 {
        let mut node = if let Some(n) = cursor.node.take() {
            match n.op {
                TraceAggregateNodeData::BranchStart { branch_end }
                    if matches!(op, TraceOp::BranchEnd)
                => {
                    // Handle empty branch
                    let TraceAggregateNodeData::BranchEnd { parent_branch_end } = branch_end.op
                    else {
                        panic!("expected branch end");
                    };
                    cursor.branch_end.set(parent_branch_end);
                    cursor.node.set(Some(branch_end));
                    return branch_end.index;
                }
                _ => {
                    *n.next.get_or_init(|| self.new_node(cursor, op))
                }
            }
        } else {
            *self.start_node
                // Initialize the start node if this is the first ever operation for this trace.
                .get_or_init(|| self.new_node(cursor, op))
        };

        let op_aggregate = op.as_op_aggregate();
        while node.op.as_op_aggregate() != op_aggregate {
            node = node.branch_sibling
                .get_or_init(|| {
                    // New branch
                    TraceAggregateBranch {
                        next: self.new_node(cursor, op),
                    }
                })
                .next;
        }

        match &node.op {
            TraceAggregateNodeData::BranchStart { branch_end } => {
                cursor.branch_end.set(Some(*branch_end));
            }
            TraceAggregateNodeData::BranchEnd { parent_branch_end } => {
                cursor.branch_end.set(*parent_branch_end);
            }
            TraceAggregateNodeData::Metric { index, .. } => {
                if let TraceOp::Metric { value, .. } = op {
                    if let Some(metric_aggregate) =
                        self.metrics.borrow_mut().get_mut(*index as usize)
                    {
                        metric_aggregate.count += 1;
                        metric_aggregate.sum += *value;
                        metric_aggregate.min = core::cmp::min(metric_aggregate.min, *value);
                        metric_aggregate.max = core::cmp::max(metric_aggregate.max, *value);
                    }
                }
            }
            _ => { }
        }

        cursor.node.set(Some(node));

        node.index
    }

    fn new_metric(&self) -> u16 {
        let mut metrics = self.metrics.borrow_mut();
        let index = metrics.len() as u16;
        metrics.push(MetricAggregate {
            count: 0,
            sum: 0,
            min: i64::MAX,
            max: i64::MIN,
        });
        index
    }

    #[inline]
    fn new_node(&self, cursor: &TraceAggregateCursor, op: &TraceOp) -> TraceAggregateNodePtr {
        let node_data = match op {
            TraceOp::CreateSource { .. } => TraceAggregateNodeData::CreateSource,
            TraceOp::DeleteSource { .. } => TraceAggregateNodeData::DeleteSource,
            TraceOp::Call { source } => TraceAggregateNodeData::Call { source: *source },
            TraceOp::PushScope => TraceAggregateNodeData::PushScope,
            TraceOp::PopScope => TraceAggregateNodeData::PopScope,
            TraceOp::BranchStart => {
                // Create both the branch start and a single branch end that all branches will
                // eventually flow into.
                let branch_end = self.nodes.push(TraceAggregateNode {
                    op: TraceAggregateNodeData::BranchEnd {
                        parent_branch_end: cursor.branch_end.take(),
                    },
                    branch_sibling: OnceCell::new(),
                    next: OnceCell::new(),
                    index: self.nodes.len() as u16,
                });
                TraceAggregateNodeData::BranchStart {
                    branch_end,
                }
            }
            TraceOp::BranchEnd => {
                // Use the existing branch end node that was created at this branch's start.
                let node = cursor.branch_end.take()
                    .expect("new probius trace aggregate branch end node");
                return node;
            }
            TraceOp::Label { label } => TraceAggregateNodeData::Label { label: unsafe { &**label } }, 
            TraceOp::Tag { .. } => TraceAggregateNodeData::Tag,
            TraceOp::Metric { name, .. } => {
                let index = self.new_metric();
                TraceAggregateNodeData::Metric { name, index }
            }

            TraceOp::LocalChannelSend { channel, .. } =>
                TraceAggregateNodeData::LocalChannelSend { channel: *channel },
            TraceOp::LocalChannelReceive { channel, .. } =>
                TraceAggregateNodeData::LocalChannelReceive { channel: *channel },
            TraceOp::LocalChannelTransferFrom { from, to, .. } =>
                TraceAggregateNodeData::LocalChannelTransferFrom { from: *from, to: *to },

            TraceOp::GlobalChannelSend { channel, .. } =>
                TraceAggregateNodeData::GlobalChannelSend { channel: *channel },
            TraceOp::GlobalChannelReceive { channel, .. } =>
                TraceAggregateNodeData::GlobalChannelReceive { channel: *channel },
            TraceOp::GlobalChannelTransferFrom { from, to, .. } =>
                TraceAggregateNodeData::GlobalChannelTransferFrom {
                    from: *from,
                    to: *to,
                },
        };

        self.nodes.push(TraceAggregateNode {
            op: node_data,
            branch_sibling: OnceCell::new(),
            next: OnceCell::new(),
            index: self.nodes.len() as u16,
        })
    }

    fn flush_full(&self, source: &Source) {
        let mut metrics = self.metrics.borrow_mut();

        source.probius.inner.trace_aggregate(
            source.next_event_id(),
            source.now_nanos(), // TODO this should be the previous flush time, not now
            &[],
            &metrics[..],
            self.nodes.iter().map(|n| {
                probius_mproto::TraceAggregateNodeGen {
                    op: n.op.as_mproto(),
                    branch_next: n.branch_sibling.get().map(|bn| bn.next.index),
                    next: n.next.get().map(|n| n.index)
                }
            }),
        );

        for node in self.nodes.iter() {
            match node.op {
                TraceAggregateNodeData::Metric { index, .. } => {
                    let index = index as usize;
                    metrics[index].count = 0;
                    metrics[index].sum = 0;
                    metrics[index].min = i64::MAX;
                    metrics[index].max = i64::MIN;
                }
                _ => {}
            }
        }
    }

    #[cfg(test)]
    fn print(&self) {
        enum Traverse {
            Node(TraceAggregateNodePtr),
            Depth(usize),
        }

        let mut branch_barriers = std::collections::HashMap::new();
        let mut next_branch_end = None;

        let indent = "  ".to_string();
        let mut depth = 0;
        let mut visit_stack = Vec::new();
        if let Some(node) = self.start_node.get().cloned() {
            visit_stack.push(Traverse::Node(node));
        }

        while let Some(traverse) = visit_stack.pop() {
            match traverse {
                Traverse::Depth(new_depth) => {
                    println!("{}-", indent.repeat(depth));
                    depth = new_depth;
                }
                Traverse::Node(node) => {
                    if let Some(sibling) = node.branch_sibling.get().map(|n| n.next.clone()) {
                        if let Some(branch_end) = next_branch_end {
                            let pending_branches = branch_barriers.get_mut(&branch_end)
                                .unwrap();
                            *pending_branches += 1;
                        }
                        visit_stack.push(Traverse::Node(sibling.clone()));
                        visit_stack.push(Traverse::Depth(depth));
                    }

                    match &node.op {
                        TraceAggregateNodeData::BranchStart { branch_end } => {
                            next_branch_end = Some(branch_end.clone());
                            branch_barriers.entry(*branch_end).or_insert(1);

                            println!(
                                "{}{:?} {:?}",
                                indent.repeat(depth),
                                node,
                                node.op.as_op_aggregate(),
                            );

                            depth += 1;
                        }
                        TraceAggregateNodeData::BranchEnd { parent_branch_end } => {
                            let branch_end = next_branch_end.clone()
                                .expect("unexpected branch end");

                            let pending_branches = branch_barriers.get_mut(&branch_end)
                                .unwrap();
                            *pending_branches -= 1;
                            if *pending_branches > 0 {
                                continue;
                            }

                            next_branch_end = parent_branch_end.clone();

                            depth -= 1;

                            println!(
                                "{}{:?} {:?}",
                                indent.repeat(depth),
                                node,
                                node.op.as_op_aggregate(),
                            );
                        }
                        _ => {
                            println!(
                                "{}{:?} {:?}",
                                indent.repeat(depth),
                                node,
                                node.op,
                            );
                        }
                    }

                    if let Some(next) = node.next.get().cloned() {
                        visit_stack.push(Traverse::Node(next));
                    }
                }
            }
        }
    }
}

impl Drop for TraceAggregator {
    fn drop(&mut self) {
        unsafe { self.nodes.unleak(); }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trace_aggregation() {
        let buffer_pool = bab::HeapBufferPool::new(8192, 4, 16);
        init(10, buffer_pool);

        enter_component("test-component", || {
            let tracer = new_trace_source("test-tracer");
            for i in 0..10 {
                tracer.trace(|| {
                    trace_metric("start", 1);
                    trace_branch(|| {
                        if i % 3 == 0 {
                            trace_metric("even", 1);
                            trace_branch(|| {
                                if i % 4 == 0 {
                                    trace_metric("% 4 = 0", 1);
                                } else if i % 4 == 1 {
                                    trace_metric("% 4 = 1", 1);
                                    trace_metric("hehe", 1);
                                } else if i % 4 == 2 {
                                    trace_metric("% 4 = 2", 1);
                                    trace_metric("haha", 1);
                                } else {
                                    trace_metric("% 4 = 3", 1);
                                }
                            });
                            trace_metric("4", 1);
                        } else {
                            trace_metric("odd", 1);
                        }
                    });
                    trace_metric("oddeven", 1);
                });
            }

            tracer.trace_aggregator.print();

            tracer.flush_aggregate_full();
        });

        for flushed_buffer in flush() {
            let len = bab::WriterFlushSender::get_complete_buffer_len(flushed_buffer) as usize;
            println!("Flushed: {:?}", unsafe { flushed_buffer.slice(0..len) });
        }
    }
}
