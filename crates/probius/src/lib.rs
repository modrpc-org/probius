pub use probius_mproto::{GlobalSourceId, MetricAggregate, SourceId};

pub use component::Component;
pub use decode::{DecodeEvents, DecodeEvent, DecodeEventBody};
pub use void_sink::init_void_sink;

#[cfg(feature = "enabled")]
pub use trace::*;
#[cfg(not(feature = "enabled"))]
pub use stubbed_trace::*;

#[cfg(feature = "tcp-sink")]
pub use tcp_sink::{init_tcp_sink, ProbiusFlusher};

mod component;
mod decode;
#[cfg(feature = "enabled")]
mod encoding;
#[cfg(feature = "enabled")]
mod link_vec;
mod void_sink;

#[cfg(feature = "enabled")]
mod trace;
#[cfg(not(feature = "enabled"))]
mod stubbed_trace;

#[cfg(feature = "tcp-sink")]
mod tcp_sink;
