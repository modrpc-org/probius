use mproto::BaseLen;

pub struct DecodeEvents<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> DecodeEvents<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, offset: 0 }
    }
}

impl<'a> Iterator for DecodeEvents<'a> {
    type Item = DecodeEvent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.buf.len() {
            return None;
        }

        let buffer_offset = self.offset;
        let event: probius_mproto::EventHeader =
            mproto::decode_value(&self.buf[buffer_offset..]).ok()?;
        self.offset += probius_mproto::EventHeader::BASE_LEN;
        let body_start = self.offset;
        self.offset += event.len as usize;
        let buffer_body_len = event.len as usize;

        match event.kind {
            probius_mproto::EventKind::CreateSource => {
                let payload = mproto::decode_value(&self.buf[body_start..]).ok()?;
                Some(DecodeEvent {
                    buffer_offset,
                    buffer_body_len,
                    kind: event.kind,
                    id: event.id,
                    body: DecodeEventBody::CreateSource(payload),
                })
            }
            probius_mproto::EventKind::DeleteSource => {
                Some(DecodeEvent {
                    buffer_offset,
                    buffer_body_len,
                    kind: event.kind,
                    id: event.id,
                    body: DecodeEventBody::DeleteSource,
                })
            }
            probius_mproto::EventKind::Trace => {
                let header = mproto::decode_value(&self.buf[body_start..]).ok()?;
                Some(DecodeEvent {
                    buffer_offset,
                    buffer_body_len,
                    kind: event.kind,
                    id: event.id,
                    body: DecodeEventBody::Trace { header },
                })
            }
            probius_mproto::EventKind::TraceAggregate => {
                let header = mproto::decode_value(&self.buf[body_start..]).ok()?;
                Some(DecodeEvent {
                    buffer_offset,
                    buffer_body_len,
                    kind: event.kind,
                    id: event.id,
                    body: DecodeEventBody::TraceAggregate { header },
                })
            }
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DecodeEvent<'a> {
    // Raw byte offset and length of this event in the `DecodeEvents` buffer
    pub buffer_offset: usize,
    pub buffer_body_len: usize,

    pub kind: probius_mproto::EventKind,
    pub id: probius_mproto::EventId,
    pub body: DecodeEventBody<'a>,
}

#[derive(Clone, Debug)]
pub enum DecodeEventBody<'a> {
    CreateSource(probius_mproto::CreateSourceLazy<'a>),
    DeleteSource,
    Trace {
        header: probius_mproto::TraceLazy<'a>,
    },
    TraceAggregate {
        header: probius_mproto::TraceAggregateLazy<'a>,
    },
    TraceAggregateDelta(probius_mproto::TraceAggregateLazy<'a>),
}

