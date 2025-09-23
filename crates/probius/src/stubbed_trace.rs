use core::future::Future;

use probius_mproto::SourceId;

#[inline]
pub fn flush() -> impl Iterator<Item = bab::BufferPtr> {
    [].into_iter()
}

#[inline]
pub fn enter_component<R>(_name: &'static str, f: impl FnOnce() -> R) -> R {
    f()
}

#[inline]
pub async fn enter_component_async<F: Future>(_name: &'static str, f: F) -> F::Output {
    f.await
}

#[inline]
pub fn enter_component_ephemeral<R>(_name: &'static str, f: impl FnOnce() -> R) -> R {
    f()
}

#[inline]
pub async fn enter_component_ephemeral_async<F: Future>(_name: &'static str, f: F) -> F::Output {
    f.await
}

#[inline]
pub fn new_trace_source(_name: &'static str) -> TraceSource {
    TraceSource(())
}

pub fn new_trace_source_ephemeral(_name: &'static str) -> TraceSource {
    TraceSource(())
}

pub struct Source(());

impl Source {
    pub fn id(&self) -> SourceId { SourceId { source: u64::MAX } }
}

pub struct TraceSource(());

impl TraceSource {
    #[inline]
    pub fn trace<R>(&self, f: impl FnOnce() -> R) -> R {
        f()
    }

    #[inline]
    pub async fn trace_future<R>(&self, f: impl core::future::Future<Output = R>) -> R {
        f.await
    }

    #[inline]
    pub fn flush_aggregate_full(&self) {
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
pub fn trace_metric(_name: &'static str, _value: i64) { }

#[inline]
pub fn trace_label(_label: &'static str) { }

#[cfg(not(feature = "enabled"))]
#[inline]
pub fn trace_branch<R>(f: impl FnOnce() -> R) -> R {
    f()
}

#[inline]
pub fn trace_branch_start() { }

#[inline]
pub fn trace_branch_end() { }

