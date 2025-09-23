use core::cell::RefCell;

use crate::SourceId;

pub struct ProbiusWriter {
    buffer_headroom: usize,
    buffer_writer: RefCell<bab::BufferWriter>,
    written_buffers: bab::BufferChain,
}

impl ProbiusWriter {
    pub fn new(buffer_headroom: usize, buffer_pool: bab::HeapBufferPool) -> Self {
        Self {
            buffer_headroom,
            buffer_writer: RefCell::new(bab::BufferWriter::new(buffer_pool)),
            written_buffers: bab::BufferChain::new(),
        }
    }

    pub fn flush(&self) -> impl Iterator<Item = bab::BufferPtr> + use<> {
        self.switch_buffer();
        self.written_buffers.drain()
    }

    fn try_write<R>(&self, len: usize, f: impl FnOnce(&mut [u8]) -> R) -> Option<R> {
        let mut headroom = 0;
        if self.buffer_writer.borrow().is_empty() {
            headroom = self.buffer_headroom;
        }

        if headroom + len > self.buffer_writer.borrow().remaining_on_buffer() {
            self.switch_buffer()?;
            headroom = self.buffer_headroom;
        }

        let mut buffer_writer = self.buffer_writer.borrow_mut();
        let write_buf = buffer_writer.try_write()?.get_mut(headroom..headroom + len)?;

        if write_buf.len() != len {
            panic!(
                "invalid write_buf len={} headroom={} remaining_on_buffer={}",
                write_buf.len(),
                self.buffer_headroom,
                self.buffer_writer.borrow().remaining_on_buffer(),
            );
        }

        let result = f(write_buf);
        buffer_writer.commit(headroom + len);
        Some(result)
    }

    fn switch_buffer(&self) -> Option<()> {
        let mut buffer_writer = self.buffer_writer.borrow_mut();
        let (buffer, written_len) = buffer_writer.next_buffer()?;
        bab::WriterFlushSender::mark_complete_buffer(buffer, written_len as u32);
        self.written_buffers.push(buffer);
        Some(())
    }

    #[inline]
    fn write_event(
        &self,
        event_id: probius_mproto::EventId,
        kind: probius_mproto::EventKind,
        payload: impl mproto::Encode,
    ) {
        let payload_len = mproto::encoded_len(&payload);
        let header = probius_mproto::EventHeader {
            id: event_id,
            len: payload_len as u16,
            kind,
        };
        let header_len = mproto::encoded_len(header);

        if
            self.try_write(header_len + payload_len, |buf| {
                mproto::encode_value(header, &mut buf[..header_len]);
                mproto::encode_value(payload, &mut buf[header_len..]);
            })
            .is_none()
        {
            // TODO record that we dropped tracing data due to insufficient buffer space.
        };
    }

    pub fn create_source(
        &self,
        event_id: probius_mproto::EventId,
        name: &str,
        parent: Option<SourceId>,
        is_recurring: bool,
    ) {
        self.write_event(
            event_id,
            probius_mproto::EventKind::CreateSource,
            probius_mproto::CreateSourceGen {
                name,
                parent,
                is_recurring,
            },
        );
    }

    pub fn delete_source(&self, event_id: probius_mproto::EventId) {
        self.write_event(event_id, probius_mproto::EventKind::DeleteSource, ());
    }

    pub fn trace_aggregate<N>(
        &self,
        event_id: probius_mproto::EventId,
        start_nanos: u64,
        counters: &[u32],
        metrics: &[probius_mproto::MetricAggregate],
        nodes: impl ExactSizeIterator<Item = N> + Clone,
    )
        where N: mproto::Encode + mproto::Compatible<probius_mproto::TraceAggregateNode>
    {
        self.write_event(
            event_id,
            probius_mproto::EventKind::TraceAggregate,
            probius_mproto::TraceAggregateGen {
                start_nanos,
                counters,
                metrics,
                nodes: mproto::ListGen(nodes),
            },
        );
    }

    pub fn trace(
        &self,
        event_id: probius_mproto::EventId,
        start_nanos: u64,
        trace: &[u8],
    ) {
        self.write_event(
            event_id,
            probius_mproto::EventKind::Trace,
            probius_mproto::TraceGen { start_nanos, trace },
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::decode::{DecodeEvents, DecodeEventBody};

    #[test]
    fn test_encoding() {
        let buffer_pool = bab::HeapBufferPool::new(8192, 4, 16);
        let headroom = 10;
        let writer = ProbiusWriter::new(headroom, buffer_pool);

        writer.create_source(
            probius_mproto::EventId {
                source: probius_mproto::SourceId { source: 42 },
                timestamp_nanos: 4200042000,
                seq: probius_mproto::EventSeq { seq: 9 },
            },
            "foobar",
            Some(probius_mproto::SourceId { source: 41 }),
            true,
        );
        writer.delete_source(probius_mproto::EventId {
            source: probius_mproto::SourceId { source: 42 },
            timestamp_nanos: 4200042001,
            seq: probius_mproto::EventSeq { seq: 0 },
        });

        for flushed_buffer in writer.flush() {
            let len = bab::WriterFlushSender::get_complete_buffer_len(flushed_buffer) as usize;
            let buf = unsafe { &flushed_buffer.slice(headroom..len) };
            let event_iter = DecodeEvents::new(buf);

            for event in event_iter {
                println!("Flushed: {:?}", event);
                if let DecodeEventBody::CreateSource(create_source) = event.body {
                    println!("name = {:?}", create_source.name());
                    println!(
                        "parent = {:?}",
                        create_source
                            .parent()
                            .unwrap()
                            .map(|p| probius_mproto::SourceId::try_from(p).unwrap())
                    );
                }
            }
        }
    }
}
