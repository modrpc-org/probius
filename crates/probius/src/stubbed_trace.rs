use core::future::Future;

use probius_mproto::SourceId;

use crate::Component;

#[inline]
pub fn flush() -> impl Iterator<Item = bab::BufferPtr> {
    [].into_iter()
}

#[inline]
pub fn new_component(_name: &str) -> Component {
    Component::new()
}

#[inline]
pub fn enter_component<R>(_name: &str, f: impl FnOnce() -> R) -> R {
    f()
}

#[inline]
pub fn enter_component_async<F: Future>(_name: &str, f: F) -> F {
    f
}

#[inline]
pub fn enter_component_ephemeral<R>(_name: &str, f: impl FnOnce() -> R) -> R {
    f()
}

#[inline]
pub fn enter_component_ephemeral_async<F: Future>(_name: &str, f: F) -> F {
    f
}

#[inline]
pub fn new_trace_source(_name: &str) -> TraceSource {
    TraceSource(())
}

#[inline]
pub fn new_trace_source_ephemeral(_name: &str) -> TraceSource {
    TraceSource(())
}

pub struct Source(());

impl Source {
    #[inline]
    pub(crate) fn new() -> Self {
        Self(())
    }

    #[inline]
    pub fn id(&self) -> SourceId { SourceId { source: u64::MAX } }
}

pub struct TraceSource(());

impl TraceSource {
    #[inline]
    pub fn trace<R>(&self, f: impl FnOnce() -> R) -> R {
        f()
    }

    #[inline]
    pub fn trace_future<F: Future>(&self, f: F) -> F {
        f
    }

    #[inline]
    pub fn flush_aggregate_full(&self) {
    }
}

/*pub fn trace_create_source(name: &str) -> Source {
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

#[inline]
pub fn trace_branch<R>(f: impl FnOnce() -> R) -> R {
    f()
}

#[inline]
pub fn trace_branch_start() { }

#[inline]
pub fn trace_branch_end() { }

