#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

use core::convert::TryFrom;
use mproto::{BaseLen, Compatible, Decode, DecodeCursor, DecodeError, DecodeResult, Encode, EncodeCursor, Lazy, Owned, max};

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SinkHandshake {
    pub app_name: String,
    pub session_id_hi: u64,
    pub session_id_lo: u64,
}

pub struct SinkHandshakeLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct SinkHandshakeGen<
    AppName: Encode + Compatible<String>,
> {
    pub app_name: AppName,
    pub session_id_hi: u64,
    pub session_id_lo: u64,
}

impl<
    AppName: Encode + Compatible<String>
> Compatible<SinkHandshake> for SinkHandshakeGen<AppName> { }
impl<
    AppName: Encode + Compatible<String>
> Compatible<SinkHandshakeGen<AppName>> for SinkHandshake { }

impl<
    AppName: Encode + Compatible<String>,
> BaseLen for SinkHandshakeGen<AppName> {
    const BASE_LEN: usize = 16 + AppName::BASE_LEN;
}

impl<
    AppName: Encode + Compatible<String>,
> Encode for SinkHandshakeGen<AppName> {
    fn scratch_len(&self) -> usize {
        self.app_name.scratch_len() + self.session_id_hi.scratch_len() + self.session_id_lo.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.app_name.encode(cursor);
        self.session_id_hi.encode(cursor);
        self.session_id_lo.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for SinkHandshake {
    type Lazy<'a> = SinkHandshakeLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for SinkHandshakeLazy<'a> {
    type Owned = SinkHandshake;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<SinkHandshakeLazy<'a>> for SinkHandshake { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<SinkHandshake> for SinkHandshakeLazy<'a> { }

impl<'a> SinkHandshakeLazy<'a> {

    pub fn app_name(&self) -> DecodeResult<&'a str> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn session_id_hi(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }

    pub fn session_id_lo(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for SinkHandshake {
    const BASE_LEN: usize = 24;
}

impl Encode for SinkHandshake {
    fn scratch_len(&self) -> usize {
        self.app_name.scratch_len() + self.session_id_hi.scratch_len() + self.session_id_lo.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.app_name.encode(cursor);
        self.session_id_hi.encode(cursor);
        self.session_id_lo.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for SinkHandshake {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let app_name = Decode::decode(cursor)?;
        let session_id_hi = Decode::decode(cursor)?;
        let session_id_lo = Decode::decode(cursor)?;

        Ok(SinkHandshake {
            app_name,
            session_id_hi,
            session_id_lo,
        })
    }
}

impl<'a> BaseLen for SinkHandshakeLazy<'a> {
    const BASE_LEN: usize = 24;
}

impl<'a> Encode for SinkHandshakeLazy<'a> {
    fn scratch_len(&self) -> usize {
        let app_name: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let session_id_hi: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let session_id_lo: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        app_name.scratch_len() + session_id_hi.scratch_len() + session_id_lo.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let app_name: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let session_id_hi: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let session_id_lo: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        app_name.encode(cursor);
        session_id_hi.encode(cursor);
        session_id_lo.encode(cursor);
    }
}

impl<'a> Decode<'a> for SinkHandshakeLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(SinkHandshakeLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<SinkHandshakeLazy<'a>> for SinkHandshake {
    type Error = DecodeError;

    fn try_from(other: SinkHandshakeLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for SinkHandshakeLazy<'a> { }

impl<'a> Clone for SinkHandshakeLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for SinkHandshakeLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SinkHandshakeLazy")
            .finish()
    }
}

impl<'a> PartialEq for SinkHandshakeLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.app_name().unwrap() == other.app_name().unwrap()
            && self.session_id_hi().unwrap() == other.session_id_hi().unwrap()&& self.session_id_lo().unwrap() == other.session_id_lo().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SourceId {
    pub source: u64,
}

pub struct SourceIdLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct SourceIdGen<> {
    pub source: u64,
}

impl<> Compatible<SourceId> for SourceIdGen<> { }
impl<> Compatible<SourceIdGen<>> for SourceId { }

impl<> BaseLen for SourceIdGen<> {
    const BASE_LEN: usize = 8;
}

impl<> Encode for SourceIdGen<> {
    fn scratch_len(&self) -> usize {
        self.source.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.source.encode(cursor);
    }
}

impl Owned for SourceId {
    type Lazy<'a> = SourceIdLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for SourceIdLazy<'a> {
    type Owned = SourceId;
}

impl<'a> Compatible<SourceIdLazy<'a>> for SourceId { }
impl<'a> Compatible<SourceId> for SourceIdLazy<'a> { }

impl<'a> SourceIdLazy<'a> {

    pub fn source(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }
}

impl BaseLen for SourceId {
    const BASE_LEN: usize = 8;
}

impl Encode for SourceId {
    fn scratch_len(&self) -> usize {
        self.source.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.source.encode(cursor);
    }
}

impl<'a> Decode<'a> for SourceId {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let source = Decode::decode(cursor)?;

        Ok(SourceId {
            source,
        })
    }
}

impl<'a> BaseLen for SourceIdLazy<'a> {
    const BASE_LEN: usize = 8;
}

impl<'a> Encode for SourceIdLazy<'a> {
    fn scratch_len(&self) -> usize {
        let source: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        source.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let source: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        source.encode(cursor);
    }
}

impl<'a> Decode<'a> for SourceIdLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(SourceIdLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<SourceIdLazy<'a>> for SourceId {
    type Error = DecodeError;

    fn try_from(other: SourceIdLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for SourceIdLazy<'a> { }

impl<'a> Clone for SourceIdLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for SourceIdLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SourceIdLazy")
            .finish()
    }
}

impl<'a> PartialEq for SourceIdLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.source().unwrap() == other.source().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct GlobalSourceId {
    pub session: u64,
    pub source: SourceId,
}

pub struct GlobalSourceIdLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct GlobalSourceIdGen<
    Source: Encode + Compatible<SourceId>,
> {
    pub session: u64,
    pub source: Source,
}

impl<
    Source: Encode + Compatible<SourceId>
> Compatible<GlobalSourceId> for GlobalSourceIdGen<Source> { }
impl<
    Source: Encode + Compatible<SourceId>
> Compatible<GlobalSourceIdGen<Source>> for GlobalSourceId { }

impl<
    Source: Encode + Compatible<SourceId>,
> BaseLen for GlobalSourceIdGen<Source> {
    const BASE_LEN: usize = 8 + Source::BASE_LEN;
}

impl<
    Source: Encode + Compatible<SourceId>,
> Encode for GlobalSourceIdGen<Source> {
    fn scratch_len(&self) -> usize {
        self.session.scratch_len() + self.source.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.session.encode(cursor);
        self.source.encode(cursor);
    }
}

impl Owned for GlobalSourceId {
    type Lazy<'a> = GlobalSourceIdLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for GlobalSourceIdLazy<'a> {
    type Owned = GlobalSourceId;
}

impl<'a> Compatible<GlobalSourceIdLazy<'a>> for GlobalSourceId { }
impl<'a> Compatible<GlobalSourceId> for GlobalSourceIdLazy<'a> { }

impl<'a> GlobalSourceIdLazy<'a> {

    pub fn session(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn source(&self) -> DecodeResult<SourceIdLazy<'a>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }
}

impl BaseLen for GlobalSourceId {
    const BASE_LEN: usize = 16;
}

impl Encode for GlobalSourceId {
    fn scratch_len(&self) -> usize {
        self.session.scratch_len() + self.source.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.session.encode(cursor);
        self.source.encode(cursor);
    }
}

impl<'a> Decode<'a> for GlobalSourceId {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let session = Decode::decode(cursor)?;
        let source = Decode::decode(cursor)?;

        Ok(GlobalSourceId {
            session,
            source,
        })
    }
}

impl<'a> BaseLen for GlobalSourceIdLazy<'a> {
    const BASE_LEN: usize = 16;
}

impl<'a> Encode for GlobalSourceIdLazy<'a> {
    fn scratch_len(&self) -> usize {
        let session: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let source: SourceIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        session.scratch_len() + source.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let session: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let source: SourceIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        session.encode(cursor);
        source.encode(cursor);
    }
}

impl<'a> Decode<'a> for GlobalSourceIdLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(GlobalSourceIdLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<GlobalSourceIdLazy<'a>> for GlobalSourceId {
    type Error = DecodeError;

    fn try_from(other: GlobalSourceIdLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for GlobalSourceIdLazy<'a> { }

impl<'a> Clone for GlobalSourceIdLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for GlobalSourceIdLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GlobalSourceIdLazy")
            .finish()
    }
}

impl<'a> PartialEq for GlobalSourceIdLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.session().unwrap() == other.session().unwrap()
            && self.source().unwrap() == other.source().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TraceCallerId {
    pub event_id: EventId,
    pub op_index: u16,
}

pub struct TraceCallerIdLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct TraceCallerIdGen<
    TEventId: Encode + Compatible<EventId>,
> {
    pub event_id: TEventId,
    pub op_index: u16,
}

impl<
    TEventId: Encode + Compatible<EventId>
> Compatible<TraceCallerId> for TraceCallerIdGen<TEventId> { }
impl<
    TEventId: Encode + Compatible<EventId>
> Compatible<TraceCallerIdGen<TEventId>> for TraceCallerId { }

impl<
    TEventId: Encode + Compatible<EventId>,
> BaseLen for TraceCallerIdGen<TEventId> {
    const BASE_LEN: usize = 2 + TEventId::BASE_LEN;
}

impl<
    TEventId: Encode + Compatible<EventId>,
> Encode for TraceCallerIdGen<TEventId> {
    fn scratch_len(&self) -> usize {
        self.event_id.scratch_len() + self.op_index.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.event_id.encode(cursor);
        self.op_index.encode(cursor);
    }
}

impl Owned for TraceCallerId {
    type Lazy<'a> = TraceCallerIdLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for TraceCallerIdLazy<'a> {
    type Owned = TraceCallerId;
}

impl<'a> Compatible<TraceCallerIdLazy<'a>> for TraceCallerId { }
impl<'a> Compatible<TraceCallerId> for TraceCallerIdLazy<'a> { }

impl<'a> TraceCallerIdLazy<'a> {

    pub fn event_id(&self) -> DecodeResult<EventIdLazy<'a>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn op_index(&self) -> DecodeResult<u16> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 18))
    }
}

impl BaseLen for TraceCallerId {
    const BASE_LEN: usize = 20;
}

impl Encode for TraceCallerId {
    fn scratch_len(&self) -> usize {
        self.event_id.scratch_len() + self.op_index.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.event_id.encode(cursor);
        self.op_index.encode(cursor);
    }
}

impl<'a> Decode<'a> for TraceCallerId {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let event_id = Decode::decode(cursor)?;
        let op_index = Decode::decode(cursor)?;

        Ok(TraceCallerId {
            event_id,
            op_index,
        })
    }
}

impl<'a> BaseLen for TraceCallerIdLazy<'a> {
    const BASE_LEN: usize = 20;
}

impl<'a> Encode for TraceCallerIdLazy<'a> {
    fn scratch_len(&self) -> usize {
        let event_id: EventIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let op_index: u16 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 18)).unwrap();
        event_id.scratch_len() + op_index.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let event_id: EventIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let op_index: u16 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 18)).unwrap();
        event_id.encode(cursor);
        op_index.encode(cursor);
    }
}

impl<'a> Decode<'a> for TraceCallerIdLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(TraceCallerIdLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<TraceCallerIdLazy<'a>> for TraceCallerId {
    type Error = DecodeError;

    fn try_from(other: TraceCallerIdLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for TraceCallerIdLazy<'a> { }

impl<'a> Clone for TraceCallerIdLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for TraceCallerIdLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TraceCallerIdLazy")
            .finish()
    }
}

impl<'a> PartialEq for TraceCallerIdLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.event_id().unwrap() == other.event_id().unwrap()
            && self.op_index().unwrap() == other.op_index().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EventSeq {
    pub seq: u16,
}

pub struct EventSeqLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct EventSeqGen<> {
    pub seq: u16,
}

impl<> Compatible<EventSeq> for EventSeqGen<> { }
impl<> Compatible<EventSeqGen<>> for EventSeq { }

impl<> BaseLen for EventSeqGen<> {
    const BASE_LEN: usize = 2;
}

impl<> Encode for EventSeqGen<> {
    fn scratch_len(&self) -> usize {
        self.seq.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.seq.encode(cursor);
    }
}

impl Owned for EventSeq {
    type Lazy<'a> = EventSeqLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for EventSeqLazy<'a> {
    type Owned = EventSeq;
}

impl<'a> Compatible<EventSeqLazy<'a>> for EventSeq { }
impl<'a> Compatible<EventSeq> for EventSeqLazy<'a> { }

impl<'a> EventSeqLazy<'a> {

    pub fn seq(&self) -> DecodeResult<u16> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }
}

impl BaseLen for EventSeq {
    const BASE_LEN: usize = 2;
}

impl Encode for EventSeq {
    fn scratch_len(&self) -> usize {
        self.seq.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.seq.encode(cursor);
    }
}

impl<'a> Decode<'a> for EventSeq {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let seq = Decode::decode(cursor)?;

        Ok(EventSeq {
            seq,
        })
    }
}

impl<'a> BaseLen for EventSeqLazy<'a> {
    const BASE_LEN: usize = 2;
}

impl<'a> Encode for EventSeqLazy<'a> {
    fn scratch_len(&self) -> usize {
        let seq: u16 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        seq.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let seq: u16 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        seq.encode(cursor);
    }
}

impl<'a> Decode<'a> for EventSeqLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(EventSeqLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<EventSeqLazy<'a>> for EventSeq {
    type Error = DecodeError;

    fn try_from(other: EventSeqLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for EventSeqLazy<'a> { }

impl<'a> Clone for EventSeqLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for EventSeqLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventSeqLazy")
            .finish()
    }
}

impl<'a> PartialEq for EventSeqLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.seq().unwrap() == other.seq().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EventId {
    pub source: SourceId,
    pub timestamp_nanos: u64,
    pub seq: EventSeq,
}

pub struct EventIdLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct EventIdGen<
    Source: Encode + Compatible<SourceId>,
    Seq: Encode + Compatible<EventSeq>,
> {
    pub source: Source,
    pub timestamp_nanos: u64,
    pub seq: Seq,
}

impl<
    Source: Encode + Compatible<SourceId>,
    Seq: Encode + Compatible<EventSeq>
> Compatible<EventId> for EventIdGen<Source, Seq> { }
impl<
    Source: Encode + Compatible<SourceId>,
    Seq: Encode + Compatible<EventSeq>
> Compatible<EventIdGen<Source, Seq>> for EventId { }

impl<
    Source: Encode + Compatible<SourceId>,
    Seq: Encode + Compatible<EventSeq>,
> BaseLen for EventIdGen<Source, Seq> {
    const BASE_LEN: usize = 8 + Source::BASE_LEN + Seq::BASE_LEN;
}

impl<
    Source: Encode + Compatible<SourceId>,
    Seq: Encode + Compatible<EventSeq>,
> Encode for EventIdGen<Source, Seq> {
    fn scratch_len(&self) -> usize {
        self.source.scratch_len() + self.timestamp_nanos.scratch_len() + self.seq.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.source.encode(cursor);
        self.timestamp_nanos.encode(cursor);
        self.seq.encode(cursor);
    }
}

impl Owned for EventId {
    type Lazy<'a> = EventIdLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for EventIdLazy<'a> {
    type Owned = EventId;
}

impl<'a> Compatible<EventIdLazy<'a>> for EventId { }
impl<'a> Compatible<EventId> for EventIdLazy<'a> { }

impl<'a> EventIdLazy<'a> {

    pub fn source(&self) -> DecodeResult<SourceIdLazy<'a>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn timestamp_nanos(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }

    pub fn seq(&self) -> DecodeResult<EventSeqLazy<'a>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16))
    }
}

impl BaseLen for EventId {
    const BASE_LEN: usize = 18;
}

impl Encode for EventId {
    fn scratch_len(&self) -> usize {
        self.source.scratch_len() + self.timestamp_nanos.scratch_len() + self.seq.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.source.encode(cursor);
        self.timestamp_nanos.encode(cursor);
        self.seq.encode(cursor);
    }
}

impl<'a> Decode<'a> for EventId {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let source = Decode::decode(cursor)?;
        let timestamp_nanos = Decode::decode(cursor)?;
        let seq = Decode::decode(cursor)?;

        Ok(EventId {
            source,
            timestamp_nanos,
            seq,
        })
    }
}

impl<'a> BaseLen for EventIdLazy<'a> {
    const BASE_LEN: usize = 18;
}

impl<'a> Encode for EventIdLazy<'a> {
    fn scratch_len(&self) -> usize {
        let source: SourceIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let timestamp_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let seq: EventSeqLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        source.scratch_len() + timestamp_nanos.scratch_len() + seq.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let source: SourceIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let timestamp_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let seq: EventSeqLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        source.encode(cursor);
        timestamp_nanos.encode(cursor);
        seq.encode(cursor);
    }
}

impl<'a> Decode<'a> for EventIdLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(EventIdLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<EventIdLazy<'a>> for EventId {
    type Error = DecodeError;

    fn try_from(other: EventIdLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for EventIdLazy<'a> { }

impl<'a> Clone for EventIdLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for EventIdLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventIdLazy")
            .finish()
    }
}

impl<'a> PartialEq for EventIdLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.source().unwrap() == other.source().unwrap()
            && self.timestamp_nanos().unwrap() == other.timestamp_nanos().unwrap()&& self.seq().unwrap() == other.seq().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EventHeader {
    pub id: EventId,
    pub len: u16,
    pub kind: EventKind,
}

pub struct EventHeaderLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct EventHeaderGen<
    Id: Encode + Compatible<EventId>,
    Kind: Encode + Compatible<EventKind>,
> {
    pub id: Id,
    pub len: u16,
    pub kind: Kind,
}

impl<
    Id: Encode + Compatible<EventId>,
    Kind: Encode + Compatible<EventKind>
> Compatible<EventHeader> for EventHeaderGen<Id, Kind> { }
impl<
    Id: Encode + Compatible<EventId>,
    Kind: Encode + Compatible<EventKind>
> Compatible<EventHeaderGen<Id, Kind>> for EventHeader { }

impl<
    Id: Encode + Compatible<EventId>,
    Kind: Encode + Compatible<EventKind>,
> BaseLen for EventHeaderGen<Id, Kind> {
    const BASE_LEN: usize = 2 + Id::BASE_LEN + Kind::BASE_LEN;
}

impl<
    Id: Encode + Compatible<EventId>,
    Kind: Encode + Compatible<EventKind>,
> Encode for EventHeaderGen<Id, Kind> {
    fn scratch_len(&self) -> usize {
        self.id.scratch_len() + self.len.scratch_len() + self.kind.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.id.encode(cursor);
        self.len.encode(cursor);
        self.kind.encode(cursor);
    }
}

impl Owned for EventHeader {
    type Lazy<'a> = EventHeaderLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for EventHeaderLazy<'a> {
    type Owned = EventHeader;
}

impl<'a> Compatible<EventHeaderLazy<'a>> for EventHeader { }
impl<'a> Compatible<EventHeader> for EventHeaderLazy<'a> { }

impl<'a> EventHeaderLazy<'a> {

    pub fn id(&self) -> DecodeResult<EventIdLazy<'a>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn len(&self) -> DecodeResult<u16> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 18))
    }

    pub fn kind(&self) -> DecodeResult<EventKindLazy> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 20))
    }
}

impl BaseLen for EventHeader {
    const BASE_LEN: usize = 21 + max(max(max(max(max(0, 0), 0), 0), 0), 0);
}

impl Encode for EventHeader {
    fn scratch_len(&self) -> usize {
        self.id.scratch_len() + self.len.scratch_len() + self.kind.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.id.encode(cursor);
        self.len.encode(cursor);
        self.kind.encode(cursor);
    }
}

impl<'a> Decode<'a> for EventHeader {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let id = Decode::decode(cursor)?;
        let len = Decode::decode(cursor)?;
        let kind = Decode::decode(cursor)?;

        Ok(EventHeader {
            id,
            len,
            kind,
        })
    }
}

impl<'a> BaseLen for EventHeaderLazy<'a> {
    const BASE_LEN: usize = 21 + max(max(max(max(max(0, 0), 0), 0), 0), 0);
}

impl<'a> Encode for EventHeaderLazy<'a> {
    fn scratch_len(&self) -> usize {
        let id: EventIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let len: u16 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 18)).unwrap();
        let kind: EventKindLazy = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 20)).unwrap();
        id.scratch_len() + len.scratch_len() + kind.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let id: EventIdLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let len: u16 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 18)).unwrap();
        let kind: EventKindLazy = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 20)).unwrap();
        id.encode(cursor);
        len.encode(cursor);
        kind.encode(cursor);
    }
}

impl<'a> Decode<'a> for EventHeaderLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(EventHeaderLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<EventHeaderLazy<'a>> for EventHeader {
    type Error = DecodeError;

    fn try_from(other: EventHeaderLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for EventHeaderLazy<'a> { }

impl<'a> Clone for EventHeaderLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for EventHeaderLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventHeaderLazy")
            .finish()
    }
}

impl<'a> PartialEq for EventHeaderLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id().unwrap() == other.id().unwrap()
            && self.len().unwrap() == other.len().unwrap()&& self.kind().unwrap() == other.kind().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum EventKind {
    CreateSource,
    DeleteSource,
    Trace,
    TraceAggregate,
    TraceAggregateDelta,
}

#[derive(Clone)]
pub enum EventKindLazy {
    CreateSource,
    DeleteSource,
    Trace,
    TraceAggregate,
    TraceAggregateDelta,
}

impl Compatible<EventKindLazy> for EventKind { }
impl Compatible<EventKind> for EventKindLazy { }

impl Owned for EventKind {
    type Lazy<'a> = EventKindLazy;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for EventKindLazy {
    type Owned = EventKind;
}

impl BaseLen for EventKind {
    const BASE_LEN: usize = 1 + max(max(max(max(max(0, 0), 0), 0), 0), 0);
}

impl Encode for EventKind {
    fn scratch_len(&self) -> usize {
        match self {
            EventKind::CreateSource => 0,
            EventKind::DeleteSource => 0,
            EventKind::Trace => 0,
            EventKind::TraceAggregate => 0,
            EventKind::TraceAggregateDelta => 0,
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            EventKind::CreateSource => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKind::DeleteSource => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKind::Trace => {
                cursor.base(1)[0] = 2;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKind::TraceAggregate => {
                cursor.base(1)[0] = 3;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKind::TraceAggregateDelta => {
                cursor.base(1)[0] = 4;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for EventKind {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKind::CreateSource)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKind::DeleteSource)
            }
            2 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKind::Trace)
            }
            3 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKind::TraceAggregate)
            }
            4 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKind::TraceAggregateDelta)
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl BaseLen for EventKindLazy {
    const BASE_LEN: usize = 1 + max(max(max(max(max(0, 0), 0), 0), 0), 0);
}

impl Encode for EventKindLazy {
    fn scratch_len(&self) -> usize {
        match self {
            EventKindLazy::CreateSource => 0,
            EventKindLazy::DeleteSource => 0,
            EventKindLazy::Trace => 0,
            EventKindLazy::TraceAggregate => 0,
            EventKindLazy::TraceAggregateDelta => 0,
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            EventKindLazy::CreateSource => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKindLazy::DeleteSource => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKindLazy::Trace => {
                cursor.base(1)[0] = 2;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKindLazy::TraceAggregate => {
                cursor.base(1)[0] = 3;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            EventKindLazy::TraceAggregateDelta => {
                cursor.base(1)[0] = 4;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for EventKindLazy {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKindLazy::CreateSource)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKindLazy::DeleteSource)
            }
            2 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKindLazy::Trace)
            }
            3 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKindLazy::TraceAggregate)
            }
            4 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(EventKindLazy::TraceAggregateDelta)
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl TryFrom<EventKindLazy> for EventKind {
    type Error = DecodeError;

    fn try_from(other: EventKindLazy) -> Result<Self, Self::Error> {
        match other {
            EventKindLazy::CreateSource => Ok(EventKind::CreateSource),
            EventKindLazy::DeleteSource => Ok(EventKind::DeleteSource),
            EventKindLazy::Trace => Ok(EventKind::Trace),
            EventKindLazy::TraceAggregate => Ok(EventKind::TraceAggregate),
            EventKindLazy::TraceAggregateDelta => Ok(EventKind::TraceAggregateDelta),
        }
    }
}

impl Copy for EventKindLazy { }

impl core::fmt::Debug for EventKindLazy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventKindLazy")
            .finish()
    }
}

impl PartialEq for EventKindLazy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EventKindLazy::CreateSource, EventKindLazy::CreateSource) => true,
            (EventKindLazy::DeleteSource, EventKindLazy::DeleteSource) => true,
            (EventKindLazy::Trace, EventKindLazy::Trace) => true,
            (EventKindLazy::TraceAggregate, EventKindLazy::TraceAggregate) => true,
            (EventKindLazy::TraceAggregateDelta, EventKindLazy::TraceAggregateDelta) => true,
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CreateSource {
    pub name: String,
    pub parent: Option<SourceId>,
    pub is_recurring: bool,
}

pub struct CreateSourceLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct CreateSourceGen<
    Name: Encode + Compatible<String>,
    Parent: Encode + Compatible<Option<SourceId>>,
> {
    pub name: Name,
    pub parent: Parent,
    pub is_recurring: bool,
}

impl<
    Name: Encode + Compatible<String>,
    Parent: Encode + Compatible<Option<SourceId>>
> Compatible<CreateSource> for CreateSourceGen<Name, Parent> { }
impl<
    Name: Encode + Compatible<String>,
    Parent: Encode + Compatible<Option<SourceId>>
> Compatible<CreateSourceGen<Name, Parent>> for CreateSource { }

impl<
    Name: Encode + Compatible<String>,
    Parent: Encode + Compatible<Option<SourceId>>,
> BaseLen for CreateSourceGen<Name, Parent> {
    const BASE_LEN: usize = 1 + Name::BASE_LEN + Parent::BASE_LEN;
}

impl<
    Name: Encode + Compatible<String>,
    Parent: Encode + Compatible<Option<SourceId>>,
> Encode for CreateSourceGen<Name, Parent> {
    fn scratch_len(&self) -> usize {
        self.name.scratch_len() + self.parent.scratch_len() + self.is_recurring.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.name.encode(cursor);
        self.parent.encode(cursor);
        self.is_recurring.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for CreateSource {
    type Lazy<'a> = CreateSourceLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for CreateSourceLazy<'a> {
    type Owned = CreateSource;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<CreateSourceLazy<'a>> for CreateSource { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<CreateSource> for CreateSourceLazy<'a> { }

impl<'a> CreateSourceLazy<'a> {

    pub fn name(&self) -> DecodeResult<&'a str> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn parent(&self) -> DecodeResult<Option<SourceIdLazy<'a>>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }

    pub fn is_recurring(&self) -> DecodeResult<bool> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 17))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for CreateSource {
    const BASE_LEN: usize = 18;
}

impl Encode for CreateSource {
    fn scratch_len(&self) -> usize {
        self.name.scratch_len() + self.parent.scratch_len() + self.is_recurring.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.name.encode(cursor);
        self.parent.encode(cursor);
        self.is_recurring.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for CreateSource {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let name = Decode::decode(cursor)?;
        let parent = Decode::decode(cursor)?;
        let is_recurring = Decode::decode(cursor)?;

        Ok(CreateSource {
            name,
            parent,
            is_recurring,
        })
    }
}

impl<'a> BaseLen for CreateSourceLazy<'a> {
    const BASE_LEN: usize = 18;
}

impl<'a> Encode for CreateSourceLazy<'a> {
    fn scratch_len(&self) -> usize {
        let name: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let parent: Option<SourceIdLazy<'a>> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let is_recurring: bool = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 17)).unwrap();
        name.scratch_len() + parent.scratch_len() + is_recurring.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let name: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let parent: Option<SourceIdLazy<'a>> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let is_recurring: bool = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 17)).unwrap();
        name.encode(cursor);
        parent.encode(cursor);
        is_recurring.encode(cursor);
    }
}

impl<'a> Decode<'a> for CreateSourceLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(CreateSourceLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<CreateSourceLazy<'a>> for CreateSource {
    type Error = DecodeError;

    fn try_from(other: CreateSourceLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for CreateSourceLazy<'a> { }

impl<'a> Clone for CreateSourceLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for CreateSourceLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CreateSourceLazy")
            .finish()
    }
}

impl<'a> PartialEq for CreateSourceLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name().unwrap() == other.name().unwrap()
            && self.parent().unwrap() == other.parent().unwrap()&& self.is_recurring().unwrap() == other.is_recurring().unwrap()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Trace {
    pub start_nanos: u64,
    pub trace: Vec<u8>,
}

pub struct TraceLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct TraceGen<
    TTrace: Encode + Compatible<Vec<u8>>,
> {
    pub start_nanos: u64,
    pub trace: TTrace,
}

impl<
    TTrace: Encode + Compatible<Vec<u8>>
> Compatible<Trace> for TraceGen<TTrace> { }
impl<
    TTrace: Encode + Compatible<Vec<u8>>
> Compatible<TraceGen<TTrace>> for Trace { }

impl<
    TTrace: Encode + Compatible<Vec<u8>>,
> BaseLen for TraceGen<TTrace> {
    const BASE_LEN: usize = 8 + TTrace::BASE_LEN;
}

impl<
    TTrace: Encode + Compatible<Vec<u8>>,
> Encode for TraceGen<TTrace> {
    fn scratch_len(&self) -> usize {
        self.start_nanos.scratch_len() + self.trace.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.start_nanos.encode(cursor);
        self.trace.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for Trace {
    type Lazy<'a> = TraceLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for TraceLazy<'a> {
    type Owned = Trace;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceLazy<'a>> for Trace { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<Trace> for TraceLazy<'a> { }

impl<'a> TraceLazy<'a> {

    pub fn start_nanos(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn trace(&self) -> DecodeResult<mproto::ListLazy<'a, u8>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for Trace {
    const BASE_LEN: usize = 16;
}

impl Encode for Trace {
    fn scratch_len(&self) -> usize {
        self.start_nanos.scratch_len() + self.trace.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.start_nanos.encode(cursor);
        self.trace.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for Trace {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let start_nanos = Decode::decode(cursor)?;
        let trace = Decode::decode(cursor)?;

        Ok(Trace {
            start_nanos,
            trace,
        })
    }
}

impl<'a> BaseLen for TraceLazy<'a> {
    const BASE_LEN: usize = 16;
}

impl<'a> Encode for TraceLazy<'a> {
    fn scratch_len(&self) -> usize {
        let start_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let trace: mproto::ListLazy<'a, u8> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        start_nanos.scratch_len() + trace.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let start_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let trace: mproto::ListLazy<'a, u8> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        start_nanos.encode(cursor);
        trace.encode(cursor);
    }
}

impl<'a> Decode<'a> for TraceLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(TraceLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<TraceLazy<'a>> for Trace {
    type Error = DecodeError;

    fn try_from(other: TraceLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for TraceLazy<'a> { }

impl<'a> Clone for TraceLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for TraceLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TraceLazy")
            .finish()
    }
}

impl<'a> PartialEq for TraceLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.start_nanos().unwrap() == other.start_nanos().unwrap()
            && self.trace().unwrap() == other.trace().unwrap()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TraceAggregate {
    pub start_nanos: u64,
    pub nodes: Vec<TraceAggregateNode>,
    pub counters: Vec<u32>,
    pub metrics: Vec<MetricAggregate>,
}

pub struct TraceAggregateLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct TraceAggregateGen<
    Nodes: Encode + Compatible<Vec<TraceAggregateNode>>,
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>,
> {
    pub start_nanos: u64,
    pub nodes: Nodes,
    pub counters: Counters,
    pub metrics: Metrics,
}

impl<
    Nodes: Encode + Compatible<Vec<TraceAggregateNode>>,
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>
> Compatible<TraceAggregate> for TraceAggregateGen<Nodes, Counters, Metrics> { }
impl<
    Nodes: Encode + Compatible<Vec<TraceAggregateNode>>,
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>
> Compatible<TraceAggregateGen<Nodes, Counters, Metrics>> for TraceAggregate { }

impl<
    Nodes: Encode + Compatible<Vec<TraceAggregateNode>>,
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>,
> BaseLen for TraceAggregateGen<Nodes, Counters, Metrics> {
    const BASE_LEN: usize = 8 + Nodes::BASE_LEN + Counters::BASE_LEN + Metrics::BASE_LEN;
}

impl<
    Nodes: Encode + Compatible<Vec<TraceAggregateNode>>,
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>,
> Encode for TraceAggregateGen<Nodes, Counters, Metrics> {
    fn scratch_len(&self) -> usize {
        self.start_nanos.scratch_len() + self.nodes.scratch_len() + self.counters.scratch_len() + self.metrics.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.start_nanos.encode(cursor);
        self.nodes.encode(cursor);
        self.counters.encode(cursor);
        self.metrics.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for TraceAggregate {
    type Lazy<'a> = TraceAggregateLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for TraceAggregateLazy<'a> {
    type Owned = TraceAggregate;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceAggregateLazy<'a>> for TraceAggregate { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceAggregate> for TraceAggregateLazy<'a> { }

impl<'a> TraceAggregateLazy<'a> {

    pub fn start_nanos(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn nodes(&self) -> DecodeResult<mproto::ListLazy<'a, TraceAggregateNode>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }

    pub fn counters(&self) -> DecodeResult<mproto::ListLazy<'a, u32>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16))
    }

    pub fn metrics(&self) -> DecodeResult<mproto::ListLazy<'a, MetricAggregate>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for TraceAggregate {
    const BASE_LEN: usize = 32;
}

impl Encode for TraceAggregate {
    fn scratch_len(&self) -> usize {
        self.start_nanos.scratch_len() + self.nodes.scratch_len() + self.counters.scratch_len() + self.metrics.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.start_nanos.encode(cursor);
        self.nodes.encode(cursor);
        self.counters.encode(cursor);
        self.metrics.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for TraceAggregate {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let start_nanos = Decode::decode(cursor)?;
        let nodes = Decode::decode(cursor)?;
        let counters = Decode::decode(cursor)?;
        let metrics = Decode::decode(cursor)?;

        Ok(TraceAggregate {
            start_nanos,
            nodes,
            counters,
            metrics,
        })
    }
}

impl<'a> BaseLen for TraceAggregateLazy<'a> {
    const BASE_LEN: usize = 32;
}

impl<'a> Encode for TraceAggregateLazy<'a> {
    fn scratch_len(&self) -> usize {
        let start_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let nodes: mproto::ListLazy<'a, TraceAggregateNode> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let counters: mproto::ListLazy<'a, u32> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        let metrics: mproto::ListLazy<'a, MetricAggregate> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24)).unwrap();
        start_nanos.scratch_len() + nodes.scratch_len() + counters.scratch_len() + metrics.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let start_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let nodes: mproto::ListLazy<'a, TraceAggregateNode> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let counters: mproto::ListLazy<'a, u32> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        let metrics: mproto::ListLazy<'a, MetricAggregate> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24)).unwrap();
        start_nanos.encode(cursor);
        nodes.encode(cursor);
        counters.encode(cursor);
        metrics.encode(cursor);
    }
}

impl<'a> Decode<'a> for TraceAggregateLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(TraceAggregateLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<TraceAggregateLazy<'a>> for TraceAggregate {
    type Error = DecodeError;

    fn try_from(other: TraceAggregateLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for TraceAggregateLazy<'a> { }

impl<'a> Clone for TraceAggregateLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for TraceAggregateLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TraceAggregateLazy")
            .finish()
    }
}

impl<'a> PartialEq for TraceAggregateLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.start_nanos().unwrap() == other.start_nanos().unwrap()
            && self.nodes().unwrap() == other.nodes().unwrap()&& self.counters().unwrap() == other.counters().unwrap()&& self.metrics().unwrap() == other.metrics().unwrap()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TraceAggregateDelta {
    pub start_nanos: u64,
    pub end_nanos: u64,
    pub counters: Vec<u32>,
    pub metrics: Vec<MetricAggregate>,
}

pub struct TraceAggregateDeltaLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct TraceAggregateDeltaGen<
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>,
> {
    pub start_nanos: u64,
    pub end_nanos: u64,
    pub counters: Counters,
    pub metrics: Metrics,
}

impl<
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>
> Compatible<TraceAggregateDelta> for TraceAggregateDeltaGen<Counters, Metrics> { }
impl<
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>
> Compatible<TraceAggregateDeltaGen<Counters, Metrics>> for TraceAggregateDelta { }

impl<
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>,
> BaseLen for TraceAggregateDeltaGen<Counters, Metrics> {
    const BASE_LEN: usize = 16 + Counters::BASE_LEN + Metrics::BASE_LEN;
}

impl<
    Counters: Encode + Compatible<Vec<u32>>,
    Metrics: Encode + Compatible<Vec<MetricAggregate>>,
> Encode for TraceAggregateDeltaGen<Counters, Metrics> {
    fn scratch_len(&self) -> usize {
        self.start_nanos.scratch_len() + self.end_nanos.scratch_len() + self.counters.scratch_len() + self.metrics.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.start_nanos.encode(cursor);
        self.end_nanos.encode(cursor);
        self.counters.encode(cursor);
        self.metrics.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for TraceAggregateDelta {
    type Lazy<'a> = TraceAggregateDeltaLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for TraceAggregateDeltaLazy<'a> {
    type Owned = TraceAggregateDelta;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceAggregateDeltaLazy<'a>> for TraceAggregateDelta { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceAggregateDelta> for TraceAggregateDeltaLazy<'a> { }

impl<'a> TraceAggregateDeltaLazy<'a> {

    pub fn start_nanos(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn end_nanos(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }

    pub fn counters(&self) -> DecodeResult<mproto::ListLazy<'a, u32>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16))
    }

    pub fn metrics(&self) -> DecodeResult<mproto::ListLazy<'a, MetricAggregate>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for TraceAggregateDelta {
    const BASE_LEN: usize = 32;
}

impl Encode for TraceAggregateDelta {
    fn scratch_len(&self) -> usize {
        self.start_nanos.scratch_len() + self.end_nanos.scratch_len() + self.counters.scratch_len() + self.metrics.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.start_nanos.encode(cursor);
        self.end_nanos.encode(cursor);
        self.counters.encode(cursor);
        self.metrics.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for TraceAggregateDelta {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let start_nanos = Decode::decode(cursor)?;
        let end_nanos = Decode::decode(cursor)?;
        let counters = Decode::decode(cursor)?;
        let metrics = Decode::decode(cursor)?;

        Ok(TraceAggregateDelta {
            start_nanos,
            end_nanos,
            counters,
            metrics,
        })
    }
}

impl<'a> BaseLen for TraceAggregateDeltaLazy<'a> {
    const BASE_LEN: usize = 32;
}

impl<'a> Encode for TraceAggregateDeltaLazy<'a> {
    fn scratch_len(&self) -> usize {
        let start_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let end_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let counters: mproto::ListLazy<'a, u32> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        let metrics: mproto::ListLazy<'a, MetricAggregate> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24)).unwrap();
        start_nanos.scratch_len() + end_nanos.scratch_len() + counters.scratch_len() + metrics.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let start_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let end_nanos: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let counters: mproto::ListLazy<'a, u32> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        let metrics: mproto::ListLazy<'a, MetricAggregate> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24)).unwrap();
        start_nanos.encode(cursor);
        end_nanos.encode(cursor);
        counters.encode(cursor);
        metrics.encode(cursor);
    }
}

impl<'a> Decode<'a> for TraceAggregateDeltaLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(TraceAggregateDeltaLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<TraceAggregateDeltaLazy<'a>> for TraceAggregateDelta {
    type Error = DecodeError;

    fn try_from(other: TraceAggregateDeltaLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for TraceAggregateDeltaLazy<'a> { }

impl<'a> Clone for TraceAggregateDeltaLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for TraceAggregateDeltaLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TraceAggregateDeltaLazy")
            .finish()
    }
}

impl<'a> PartialEq for TraceAggregateDeltaLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.start_nanos().unwrap() == other.start_nanos().unwrap()
            && self.end_nanos().unwrap() == other.end_nanos().unwrap()&& self.counters().unwrap() == other.counters().unwrap()&& self.metrics().unwrap() == other.metrics().unwrap()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TraceAggregateNode {
    pub op: TraceOpAggregate,
    pub branch_next: Option<u16>,
    pub next: Option<u16>,
}

pub struct TraceAggregateNodeLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct TraceAggregateNodeGen<
    Op: Encode + Compatible<TraceOpAggregate>,
    BranchNext: Encode + Compatible<Option<u16>>,
    Next: Encode + Compatible<Option<u16>>,
> {
    pub op: Op,
    pub branch_next: BranchNext,
    pub next: Next,
}

impl<
    Op: Encode + Compatible<TraceOpAggregate>,
    BranchNext: Encode + Compatible<Option<u16>>,
    Next: Encode + Compatible<Option<u16>>
> Compatible<TraceAggregateNode> for TraceAggregateNodeGen<Op, BranchNext, Next> { }
impl<
    Op: Encode + Compatible<TraceOpAggregate>,
    BranchNext: Encode + Compatible<Option<u16>>,
    Next: Encode + Compatible<Option<u16>>
> Compatible<TraceAggregateNodeGen<Op, BranchNext, Next>> for TraceAggregateNode { }

impl<
    Op: Encode + Compatible<TraceOpAggregate>,
    BranchNext: Encode + Compatible<Option<u16>>,
    Next: Encode + Compatible<Option<u16>>,
> BaseLen for TraceAggregateNodeGen<Op, BranchNext, Next> {
    const BASE_LEN: usize = Op::BASE_LEN + BranchNext::BASE_LEN + Next::BASE_LEN;
}

impl<
    Op: Encode + Compatible<TraceOpAggregate>,
    BranchNext: Encode + Compatible<Option<u16>>,
    Next: Encode + Compatible<Option<u16>>,
> Encode for TraceAggregateNodeGen<Op, BranchNext, Next> {
    fn scratch_len(&self) -> usize {
        self.op.scratch_len() + self.branch_next.scratch_len() + self.next.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.op.encode(cursor);
        self.branch_next.encode(cursor);
        self.next.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for TraceAggregateNode {
    type Lazy<'a> = TraceAggregateNodeLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for TraceAggregateNodeLazy<'a> {
    type Owned = TraceAggregateNode;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceAggregateNodeLazy<'a>> for TraceAggregateNode { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceAggregateNode> for TraceAggregateNodeLazy<'a> { }

impl<'a> TraceAggregateNodeLazy<'a> {

    pub fn op(&self) -> DecodeResult<TraceOpAggregateLazy<'a>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn branch_next(&self) -> DecodeResult<Option<u16>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 1 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32)))
    }

    pub fn next(&self) -> DecodeResult<Option<u16>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 4 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32)))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for TraceAggregateNode {
    const BASE_LEN: usize = 7 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32);
}

impl Encode for TraceAggregateNode {
    fn scratch_len(&self) -> usize {
        self.op.scratch_len() + self.branch_next.scratch_len() + self.next.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.op.encode(cursor);
        self.branch_next.encode(cursor);
        self.next.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for TraceAggregateNode {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let op = Decode::decode(cursor)?;
        let branch_next = Decode::decode(cursor)?;
        let next = Decode::decode(cursor)?;

        Ok(TraceAggregateNode {
            op,
            branch_next,
            next,
        })
    }
}

impl<'a> BaseLen for TraceAggregateNodeLazy<'a> {
    const BASE_LEN: usize = 7 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32);
}

impl<'a> Encode for TraceAggregateNodeLazy<'a> {
    fn scratch_len(&self) -> usize {
        let op: TraceOpAggregateLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let branch_next: Option<u16> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 1 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32))).unwrap();
        let next: Option<u16> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 4 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32))).unwrap();
        op.scratch_len() + branch_next.scratch_len() + next.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let op: TraceOpAggregateLazy<'a> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let branch_next: Option<u16> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 1 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32))).unwrap();
        let next: Option<u16> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 4 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32))).unwrap();
        op.encode(cursor);
        branch_next.encode(cursor);
        next.encode(cursor);
    }
}

impl<'a> Decode<'a> for TraceAggregateNodeLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(TraceAggregateNodeLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<TraceAggregateNodeLazy<'a>> for TraceAggregateNode {
    type Error = DecodeError;

    fn try_from(other: TraceAggregateNodeLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for TraceAggregateNodeLazy<'a> { }

impl<'a> Clone for TraceAggregateNodeLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for TraceAggregateNodeLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TraceAggregateNodeLazy")
            .finish()
    }
}

impl<'a> PartialEq for TraceAggregateNodeLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.op().unwrap() == other.op().unwrap()
            && self.branch_next().unwrap() == other.branch_next().unwrap()&& self.next().unwrap() == other.next().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TraceOp {
    CreateSource {
        source: SourceId,
    },
    DeleteSource {
        source: SourceId,
    },
    Call {
        source: SourceId,
    },
    PushScope,
    PopScope,
    BranchStart,
    BranchEnd,
    Label,
    Tag {
        tag: u64,
    },
    Metric {
        value: i64,
    },
    ChannelSend {
        channel: SourceId,
    },
    ChannelReceive {
        channel: SourceId,
        sender: TraceCallerId,
    },
    ChannelTransfer {
        from: SourceId,
        to: SourceId,
    },
    GlobalChannelSend {
        channel: GlobalSourceId,
    },
    GlobalChannelReceive {
        channel: GlobalSourceId,
    },
    GlobalChannelTransfer {
        from: GlobalSourceId,
        to: GlobalSourceId,
    },
}

#[derive(Clone)]
pub enum TraceOpLazy<'a> {
    CreateSource {
        source: SourceIdLazy<'a>,
    },
    DeleteSource {
        source: SourceIdLazy<'a>,
    },
    Call {
        source: SourceIdLazy<'a>,
    },
    PushScope,
    PopScope,
    BranchStart,
    BranchEnd,
    Label,
    Tag {
        tag: u64,
    },
    Metric {
        value: i64,
    },
    ChannelSend {
        channel: SourceIdLazy<'a>,
    },
    ChannelReceive {
        channel: SourceIdLazy<'a>,
        sender: TraceCallerIdLazy<'a>,
    },
    ChannelTransfer {
        from: SourceIdLazy<'a>,
        to: SourceIdLazy<'a>,
    },
    GlobalChannelSend {
        channel: GlobalSourceIdLazy<'a>,
    },
    GlobalChannelReceive {
        channel: GlobalSourceIdLazy<'a>,
    },
    GlobalChannelTransfer {
        from: GlobalSourceIdLazy<'a>,
        to: GlobalSourceIdLazy<'a>,
    },
}

impl<'a> Compatible<TraceOpLazy<'a>> for TraceOp { }
impl<'a> Compatible<TraceOp> for TraceOpLazy<'a> { }

impl Owned for TraceOp {
    type Lazy<'a> = TraceOpLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for TraceOpLazy<'a> {
    type Owned = TraceOp;
}

impl BaseLen for TraceOp {
    const BASE_LEN: usize = 1 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 8), 8), 8), 0), 0), 0), 0), 0), 8), 8), 8), 28), 16), 16), 16), 32);
}

impl Encode for TraceOp {
    fn scratch_len(&self) -> usize {
        match self {
            TraceOp::CreateSource { source } => {
                source.scratch_len()
            }
            TraceOp::DeleteSource { source } => {
                source.scratch_len()
            }
            TraceOp::Call { source } => {
                source.scratch_len()
            }
            TraceOp::PushScope => 0,
            TraceOp::PopScope => 0,
            TraceOp::BranchStart => 0,
            TraceOp::BranchEnd => 0,
            TraceOp::Label => 0,
            TraceOp::Tag { tag } => {
                tag.scratch_len()
            }
            TraceOp::Metric { value } => {
                value.scratch_len()
            }
            TraceOp::ChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOp::ChannelReceive { channel, sender } => {
                channel.scratch_len() + sender.scratch_len()
            }
            TraceOp::ChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
            TraceOp::GlobalChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOp::GlobalChannelReceive { channel } => {
                channel.scratch_len()
            }
            TraceOp::GlobalChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            TraceOp::CreateSource { source } => {
                cursor.base(1)[0] = 0;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOp::DeleteSource { source } => {
                cursor.base(1)[0] = 1;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOp::Call { source } => {
                cursor.base(1)[0] = 2;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOp::PushScope => {
                cursor.base(1)[0] = 3;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOp::PopScope => {
                cursor.base(1)[0] = 4;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOp::BranchStart => {
                cursor.base(1)[0] = 5;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOp::BranchEnd => {
                cursor.base(1)[0] = 6;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOp::Label => {
                cursor.base(1)[0] = 7;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOp::Tag { tag } => {
                cursor.base(1)[0] = 8;
                tag.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOp::Metric { value } => {
                cursor.base(1)[0] = 9;
                value.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOp::ChannelSend { channel } => {
                cursor.base(1)[0] = 10;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOp::ChannelReceive { channel, sender } => {
                cursor.base(1)[0] = 11;
                channel.encode(cursor);
                sender.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (28)).fill(0);
            }
            TraceOp::ChannelTransfer { from, to } => {
                cursor.base(1)[0] = 12;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOp::GlobalChannelSend { channel } => {
                cursor.base(1)[0] = 13;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOp::GlobalChannelReceive { channel } => {
                cursor.base(1)[0] = 14;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOp::GlobalChannelTransfer { from, to } => {
                cursor.base(1)[0] = 15;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (32)).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for TraceOp {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOp::CreateSource {
                    source,
                })
            }
            1 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOp::DeleteSource {
                    source,
                })
            }
            2 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOp::Call {
                    source,
                })
            }
            3 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOp::PushScope)
            }
            4 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOp::PopScope)
            }
            5 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOp::BranchStart)
            }
            6 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOp::BranchEnd)
            }
            7 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOp::Label)
            }
            8 => {
                let tag = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOp::Tag {
                    tag,
                })
            }
            9 => {
                let value = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOp::Metric {
                    value,
                })
            }
            10 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOp::ChannelSend {
                    channel,
                })
            }
            11 => {
                let channel = Decode::decode(cursor)?;
                let sender = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (28));
                Ok(TraceOp::ChannelReceive {
                    channel,
                    sender,
                })
            }
            12 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOp::ChannelTransfer {
                    from,
                    to,
                })
            }
            13 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOp::GlobalChannelSend {
                    channel,
                })
            }
            14 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOp::GlobalChannelReceive {
                    channel,
                })
            }
            15 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (32));
                Ok(TraceOp::GlobalChannelTransfer {
                    from,
                    to,
                })
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl<'a> BaseLen for TraceOpLazy<'a> {
    const BASE_LEN: usize = 1 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 8), 8), 8), 0), 0), 0), 0), 0), 8), 8), 8), 28), 16), 16), 16), 32);
}

impl<'a> Encode for TraceOpLazy<'a> {
    fn scratch_len(&self) -> usize {
        match self {
            TraceOpLazy::CreateSource { source } => {
                source.scratch_len()
            }
            TraceOpLazy::DeleteSource { source } => {
                source.scratch_len()
            }
            TraceOpLazy::Call { source } => {
                source.scratch_len()
            }
            TraceOpLazy::PushScope => 0,
            TraceOpLazy::PopScope => 0,
            TraceOpLazy::BranchStart => 0,
            TraceOpLazy::BranchEnd => 0,
            TraceOpLazy::Label => 0,
            TraceOpLazy::Tag { tag } => {
                tag.scratch_len()
            }
            TraceOpLazy::Metric { value } => {
                value.scratch_len()
            }
            TraceOpLazy::ChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOpLazy::ChannelReceive { channel, sender } => {
                channel.scratch_len() + sender.scratch_len()
            }
            TraceOpLazy::ChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
            TraceOpLazy::GlobalChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOpLazy::GlobalChannelReceive { channel } => {
                channel.scratch_len()
            }
            TraceOpLazy::GlobalChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            TraceOpLazy::CreateSource { source } => {
                cursor.base(1)[0] = 0;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpLazy::DeleteSource { source } => {
                cursor.base(1)[0] = 1;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpLazy::Call { source } => {
                cursor.base(1)[0] = 2;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpLazy::PushScope => {
                cursor.base(1)[0] = 3;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpLazy::PopScope => {
                cursor.base(1)[0] = 4;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpLazy::BranchStart => {
                cursor.base(1)[0] = 5;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpLazy::BranchEnd => {
                cursor.base(1)[0] = 6;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpLazy::Label => {
                cursor.base(1)[0] = 7;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpLazy::Tag { tag } => {
                cursor.base(1)[0] = 8;
                tag.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpLazy::Metric { value } => {
                cursor.base(1)[0] = 9;
                value.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpLazy::ChannelSend { channel } => {
                cursor.base(1)[0] = 10;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpLazy::ChannelReceive { channel, sender } => {
                cursor.base(1)[0] = 11;
                channel.encode(cursor);
                sender.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (28)).fill(0);
            }
            TraceOpLazy::ChannelTransfer { from, to } => {
                cursor.base(1)[0] = 12;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpLazy::GlobalChannelSend { channel } => {
                cursor.base(1)[0] = 13;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpLazy::GlobalChannelReceive { channel } => {
                cursor.base(1)[0] = 14;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpLazy::GlobalChannelTransfer { from, to } => {
                cursor.base(1)[0] = 15;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (32)).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for TraceOpLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpLazy::CreateSource {
                    source,
                })
            }
            1 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpLazy::DeleteSource {
                    source,
                })
            }
            2 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpLazy::Call {
                    source,
                })
            }
            3 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpLazy::PushScope)
            }
            4 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpLazy::PopScope)
            }
            5 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpLazy::BranchStart)
            }
            6 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpLazy::BranchEnd)
            }
            7 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpLazy::Label)
            }
            8 => {
                let tag = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpLazy::Tag {
                    tag,
                })
            }
            9 => {
                let value = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpLazy::Metric {
                    value,
                })
            }
            10 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpLazy::ChannelSend {
                    channel,
                })
            }
            11 => {
                let channel = Decode::decode(cursor)?;
                let sender = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (28));
                Ok(TraceOpLazy::ChannelReceive {
                    channel,
                    sender,
                })
            }
            12 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpLazy::ChannelTransfer {
                    from,
                    to,
                })
            }
            13 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpLazy::GlobalChannelSend {
                    channel,
                })
            }
            14 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpLazy::GlobalChannelReceive {
                    channel,
                })
            }
            15 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (32));
                Ok(TraceOpLazy::GlobalChannelTransfer {
                    from,
                    to,
                })
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl<'a> TryFrom<TraceOpLazy<'a>> for TraceOp {
    type Error = DecodeError;

    fn try_from(other: TraceOpLazy<'a>) -> Result<Self, Self::Error> {
        match other {
            TraceOpLazy::CreateSource { source, } => {
                Ok(TraceOp::CreateSource {
                    source: Owned::lazy_to_owned(source)?,
                })
            }
            TraceOpLazy::DeleteSource { source, } => {
                Ok(TraceOp::DeleteSource {
                    source: Owned::lazy_to_owned(source)?,
                })
            }
            TraceOpLazy::Call { source, } => {
                Ok(TraceOp::Call {
                    source: Owned::lazy_to_owned(source)?,
                })
            }
            TraceOpLazy::PushScope => Ok(TraceOp::PushScope),
            TraceOpLazy::PopScope => Ok(TraceOp::PopScope),
            TraceOpLazy::BranchStart => Ok(TraceOp::BranchStart),
            TraceOpLazy::BranchEnd => Ok(TraceOp::BranchEnd),
            TraceOpLazy::Label => Ok(TraceOp::Label),
            TraceOpLazy::Tag { tag, } => {
                Ok(TraceOp::Tag {
                    tag: Owned::lazy_to_owned(tag)?,
                })
            }
            TraceOpLazy::Metric { value, } => {
                Ok(TraceOp::Metric {
                    value: Owned::lazy_to_owned(value)?,
                })
            }
            TraceOpLazy::ChannelSend { channel, } => {
                Ok(TraceOp::ChannelSend {
                    channel: Owned::lazy_to_owned(channel)?,
                })
            }
            TraceOpLazy::ChannelReceive { channel,sender, } => {
                Ok(TraceOp::ChannelReceive {
                    channel: Owned::lazy_to_owned(channel)?,
                    sender: Owned::lazy_to_owned(sender)?,
                })
            }
            TraceOpLazy::ChannelTransfer { from,to, } => {
                Ok(TraceOp::ChannelTransfer {
                    from: Owned::lazy_to_owned(from)?,
                    to: Owned::lazy_to_owned(to)?,
                })
            }
            TraceOpLazy::GlobalChannelSend { channel, } => {
                Ok(TraceOp::GlobalChannelSend {
                    channel: Owned::lazy_to_owned(channel)?,
                })
            }
            TraceOpLazy::GlobalChannelReceive { channel, } => {
                Ok(TraceOp::GlobalChannelReceive {
                    channel: Owned::lazy_to_owned(channel)?,
                })
            }
            TraceOpLazy::GlobalChannelTransfer { from,to, } => {
                Ok(TraceOp::GlobalChannelTransfer {
                    from: Owned::lazy_to_owned(from)?,
                    to: Owned::lazy_to_owned(to)?,
                })
            }
        }
    }
}

impl<'a> Copy for TraceOpLazy<'a> { }

impl<'a> core::fmt::Debug for TraceOpLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TraceOpLazy")
            .finish()
    }
}

impl<'a> PartialEq for TraceOpLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TraceOpLazy::CreateSource {
                    source: self_source
                },
                TraceOpLazy::CreateSource {
                    source: other_source
                },
            ) => {
                self_source == other_source
            }
            (
                TraceOpLazy::DeleteSource {
                    source: self_source
                },
                TraceOpLazy::DeleteSource {
                    source: other_source
                },
            ) => {
                self_source == other_source
            }
            (
                TraceOpLazy::Call {
                    source: self_source
                },
                TraceOpLazy::Call {
                    source: other_source
                },
            ) => {
                self_source == other_source
            }
            (TraceOpLazy::PushScope, TraceOpLazy::PushScope) => true,
            (TraceOpLazy::PopScope, TraceOpLazy::PopScope) => true,
            (TraceOpLazy::BranchStart, TraceOpLazy::BranchStart) => true,
            (TraceOpLazy::BranchEnd, TraceOpLazy::BranchEnd) => true,
            (TraceOpLazy::Label, TraceOpLazy::Label) => true,
            (
                TraceOpLazy::Tag {
                    tag: self_tag
                },
                TraceOpLazy::Tag {
                    tag: other_tag
                },
            ) => {
                self_tag == other_tag
            }
            (
                TraceOpLazy::Metric {
                    value: self_value
                },
                TraceOpLazy::Metric {
                    value: other_value
                },
            ) => {
                self_value == other_value
            }
            (
                TraceOpLazy::ChannelSend {
                    channel: self_channel
                },
                TraceOpLazy::ChannelSend {
                    channel: other_channel
                },
            ) => {
                self_channel == other_channel
            }
            (
                TraceOpLazy::ChannelReceive {
                    channel: self_channel, sender: self_sender
                },
                TraceOpLazy::ChannelReceive {
                    channel: other_channel, sender: other_sender
                },
            ) => {
                self_channel == other_channel
                    && self_sender == other_sender
            }
            (
                TraceOpLazy::ChannelTransfer {
                    from: self_from, to: self_to
                },
                TraceOpLazy::ChannelTransfer {
                    from: other_from, to: other_to
                },
            ) => {
                self_from == other_from
                    && self_to == other_to
            }
            (
                TraceOpLazy::GlobalChannelSend {
                    channel: self_channel
                },
                TraceOpLazy::GlobalChannelSend {
                    channel: other_channel
                },
            ) => {
                self_channel == other_channel
            }
            (
                TraceOpLazy::GlobalChannelReceive {
                    channel: self_channel
                },
                TraceOpLazy::GlobalChannelReceive {
                    channel: other_channel
                },
            ) => {
                self_channel == other_channel
            }
            (
                TraceOpLazy::GlobalChannelTransfer {
                    from: self_from, to: self_to
                },
                TraceOpLazy::GlobalChannelTransfer {
                    from: other_from, to: other_to
                },
            ) => {
                self_from == other_from
                    && self_to == other_to
            }
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TraceOpAggregate {
    CreateSource,
    DeleteSource,
    Call {
        source: SourceId,
    },
    PushScope,
    PopScope,
    BranchStart {
        branch_end: u16,
    },
    BranchEnd {
        parent_branch_end: u16,
    },
    Label {
        label: String,
    },
    Tag,
    Metric {
        name: String,
        index: u16,
    },
    ChannelSend {
        channel: SourceId,
    },
    ChannelReceive {
        channel: SourceId,
    },
    ChannelTransfer {
        from: SourceId,
        to: SourceId,
    },
    GlobalChannelSend {
        channel: GlobalSourceId,
    },
    GlobalChannelReceive {
        channel: GlobalSourceId,
    },
    GlobalChannelTransfer {
        from: GlobalSourceId,
        to: GlobalSourceId,
    },
}

#[derive(Clone)]
pub enum TraceOpAggregateLazy<'a> {
    CreateSource,
    DeleteSource,
    Call {
        source: SourceIdLazy<'a>,
    },
    PushScope,
    PopScope,
    BranchStart {
        branch_end: u16,
    },
    BranchEnd {
        parent_branch_end: u16,
    },
    Label {
        label: &'a str,
    },
    Tag,
    Metric {
        name: &'a str,
        index: u16,
    },
    ChannelSend {
        channel: SourceIdLazy<'a>,
    },
    ChannelReceive {
        channel: SourceIdLazy<'a>,
    },
    ChannelTransfer {
        from: SourceIdLazy<'a>,
        to: SourceIdLazy<'a>,
    },
    GlobalChannelSend {
        channel: GlobalSourceIdLazy<'a>,
    },
    GlobalChannelReceive {
        channel: GlobalSourceIdLazy<'a>,
    },
    GlobalChannelTransfer {
        from: GlobalSourceIdLazy<'a>,
        to: GlobalSourceIdLazy<'a>,
    },
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceOpAggregateLazy<'a>> for TraceOpAggregate { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<TraceOpAggregate> for TraceOpAggregateLazy<'a> { }

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for TraceOpAggregate {
    type Lazy<'a> = TraceOpAggregateLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for TraceOpAggregateLazy<'a> {
    type Owned = TraceOpAggregate;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for TraceOpAggregate {
    const BASE_LEN: usize = 1 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32);
}

impl Encode for TraceOpAggregate {
    fn scratch_len(&self) -> usize {
        match self {
            TraceOpAggregate::CreateSource => 0,
            TraceOpAggregate::DeleteSource => 0,
            TraceOpAggregate::Call { source } => {
                source.scratch_len()
            }
            TraceOpAggregate::PushScope => 0,
            TraceOpAggregate::PopScope => 0,
            TraceOpAggregate::BranchStart { branch_end } => {
                branch_end.scratch_len()
            }
            TraceOpAggregate::BranchEnd { parent_branch_end } => {
                parent_branch_end.scratch_len()
            }
            TraceOpAggregate::Label { label } => {
                label.scratch_len()
            }
            TraceOpAggregate::Tag => 0,
            TraceOpAggregate::Metric { name, index } => {
                name.scratch_len() + index.scratch_len()
            }
            TraceOpAggregate::ChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregate::ChannelReceive { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregate::ChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
            TraceOpAggregate::GlobalChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregate::GlobalChannelReceive { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregate::GlobalChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            TraceOpAggregate::CreateSource => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregate::DeleteSource => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregate::Call { source } => {
                cursor.base(1)[0] = 2;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregate::PushScope => {
                cursor.base(1)[0] = 3;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregate::PopScope => {
                cursor.base(1)[0] = 4;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregate::BranchStart { branch_end } => {
                cursor.base(1)[0] = 5;
                branch_end.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (2)).fill(0);
            }
            TraceOpAggregate::BranchEnd { parent_branch_end } => {
                cursor.base(1)[0] = 6;
                parent_branch_end.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (2)).fill(0);
            }
            TraceOpAggregate::Label { label } => {
                cursor.base(1)[0] = 7;
                label.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregate::Tag => {
                cursor.base(1)[0] = 8;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregate::Metric { name, index } => {
                cursor.base(1)[0] = 9;
                name.encode(cursor);
                index.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (10)).fill(0);
            }
            TraceOpAggregate::ChannelSend { channel } => {
                cursor.base(1)[0] = 10;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregate::ChannelReceive { channel } => {
                cursor.base(1)[0] = 11;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregate::ChannelTransfer { from, to } => {
                cursor.base(1)[0] = 12;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpAggregate::GlobalChannelSend { channel } => {
                cursor.base(1)[0] = 13;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpAggregate::GlobalChannelReceive { channel } => {
                cursor.base(1)[0] = 14;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpAggregate::GlobalChannelTransfer { from, to } => {
                cursor.base(1)[0] = 15;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (32)).fill(0);
            }
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for TraceOpAggregate {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregate::CreateSource)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregate::DeleteSource)
            }
            2 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregate::Call {
                    source,
                })
            }
            3 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregate::PushScope)
            }
            4 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregate::PopScope)
            }
            5 => {
                let branch_end = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (2));
                Ok(TraceOpAggregate::BranchStart {
                    branch_end,
                })
            }
            6 => {
                let parent_branch_end = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (2));
                Ok(TraceOpAggregate::BranchEnd {
                    parent_branch_end,
                })
            }
            7 => {
                let label = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregate::Label {
                    label,
                })
            }
            8 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregate::Tag)
            }
            9 => {
                let name = Decode::decode(cursor)?;
                let index = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (10));
                Ok(TraceOpAggregate::Metric {
                    name,
                    index,
                })
            }
            10 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregate::ChannelSend {
                    channel,
                })
            }
            11 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregate::ChannelReceive {
                    channel,
                })
            }
            12 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpAggregate::ChannelTransfer {
                    from,
                    to,
                })
            }
            13 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpAggregate::GlobalChannelSend {
                    channel,
                })
            }
            14 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpAggregate::GlobalChannelReceive {
                    channel,
                })
            }
            15 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (32));
                Ok(TraceOpAggregate::GlobalChannelTransfer {
                    from,
                    to,
                })
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl<'a> BaseLen for TraceOpAggregateLazy<'a> {
    const BASE_LEN: usize = 1 + max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(max(0, 0), 0), 8), 0), 0), 2), 2), 8), 0), 10), 8), 8), 16), 16), 16), 32);
}

impl<'a> Encode for TraceOpAggregateLazy<'a> {
    fn scratch_len(&self) -> usize {
        match self {
            TraceOpAggregateLazy::CreateSource => 0,
            TraceOpAggregateLazy::DeleteSource => 0,
            TraceOpAggregateLazy::Call { source } => {
                source.scratch_len()
            }
            TraceOpAggregateLazy::PushScope => 0,
            TraceOpAggregateLazy::PopScope => 0,
            TraceOpAggregateLazy::BranchStart { branch_end } => {
                branch_end.scratch_len()
            }
            TraceOpAggregateLazy::BranchEnd { parent_branch_end } => {
                parent_branch_end.scratch_len()
            }
            TraceOpAggregateLazy::Label { label } => {
                label.scratch_len()
            }
            TraceOpAggregateLazy::Tag => 0,
            TraceOpAggregateLazy::Metric { name, index } => {
                name.scratch_len() + index.scratch_len()
            }
            TraceOpAggregateLazy::ChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregateLazy::ChannelReceive { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregateLazy::ChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
            TraceOpAggregateLazy::GlobalChannelSend { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregateLazy::GlobalChannelReceive { channel } => {
                channel.scratch_len()
            }
            TraceOpAggregateLazy::GlobalChannelTransfer { from, to } => {
                from.scratch_len() + to.scratch_len()
            }
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            TraceOpAggregateLazy::CreateSource => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregateLazy::DeleteSource => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregateLazy::Call { source } => {
                cursor.base(1)[0] = 2;
                source.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregateLazy::PushScope => {
                cursor.base(1)[0] = 3;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregateLazy::PopScope => {
                cursor.base(1)[0] = 4;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregateLazy::BranchStart { branch_end } => {
                cursor.base(1)[0] = 5;
                branch_end.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (2)).fill(0);
            }
            TraceOpAggregateLazy::BranchEnd { parent_branch_end } => {
                cursor.base(1)[0] = 6;
                parent_branch_end.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (2)).fill(0);
            }
            TraceOpAggregateLazy::Label { label } => {
                cursor.base(1)[0] = 7;
                label.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregateLazy::Tag => {
                cursor.base(1)[0] = 8;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            TraceOpAggregateLazy::Metric { name, index } => {
                cursor.base(1)[0] = 9;
                name.encode(cursor);
                index.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (10)).fill(0);
            }
            TraceOpAggregateLazy::ChannelSend { channel } => {
                cursor.base(1)[0] = 10;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregateLazy::ChannelReceive { channel } => {
                cursor.base(1)[0] = 11;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (8)).fill(0);
            }
            TraceOpAggregateLazy::ChannelTransfer { from, to } => {
                cursor.base(1)[0] = 12;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpAggregateLazy::GlobalChannelSend { channel } => {
                cursor.base(1)[0] = 13;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpAggregateLazy::GlobalChannelReceive { channel } => {
                cursor.base(1)[0] = 14;
                channel.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (16)).fill(0);
            }
            TraceOpAggregateLazy::GlobalChannelTransfer { from, to } => {
                cursor.base(1)[0] = 15;
                from.encode(cursor);
                to.encode(cursor);
                cursor.base(Self::BASE_LEN - 1 - (32)).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for TraceOpAggregateLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregateLazy::CreateSource)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregateLazy::DeleteSource)
            }
            2 => {
                let source = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregateLazy::Call {
                    source,
                })
            }
            3 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregateLazy::PushScope)
            }
            4 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregateLazy::PopScope)
            }
            5 => {
                let branch_end = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (2));
                Ok(TraceOpAggregateLazy::BranchStart {
                    branch_end,
                })
            }
            6 => {
                let parent_branch_end = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (2));
                Ok(TraceOpAggregateLazy::BranchEnd {
                    parent_branch_end,
                })
            }
            7 => {
                let label = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregateLazy::Label {
                    label,
                })
            }
            8 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(TraceOpAggregateLazy::Tag)
            }
            9 => {
                let name = Decode::decode(cursor)?;
                let index = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (10));
                Ok(TraceOpAggregateLazy::Metric {
                    name,
                    index,
                })
            }
            10 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregateLazy::ChannelSend {
                    channel,
                })
            }
            11 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (8));
                Ok(TraceOpAggregateLazy::ChannelReceive {
                    channel,
                })
            }
            12 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpAggregateLazy::ChannelTransfer {
                    from,
                    to,
                })
            }
            13 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpAggregateLazy::GlobalChannelSend {
                    channel,
                })
            }
            14 => {
                let channel = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (16));
                Ok(TraceOpAggregateLazy::GlobalChannelReceive {
                    channel,
                })
            }
            15 => {
                let from = Decode::decode(cursor)?;
                let to = Decode::decode(cursor)?;
                cursor.advance(Self::BASE_LEN - 1 - (32));
                Ok(TraceOpAggregateLazy::GlobalChannelTransfer {
                    from,
                    to,
                })
            }
            _ => { Err(DecodeError) }
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<TraceOpAggregateLazy<'a>> for TraceOpAggregate {
    type Error = DecodeError;

    fn try_from(other: TraceOpAggregateLazy<'a>) -> Result<Self, Self::Error> {
        match other {
            TraceOpAggregateLazy::CreateSource => Ok(TraceOpAggregate::CreateSource),
            TraceOpAggregateLazy::DeleteSource => Ok(TraceOpAggregate::DeleteSource),
            TraceOpAggregateLazy::Call { source, } => {
                Ok(TraceOpAggregate::Call {
                    source: Owned::lazy_to_owned(source)?,
                })
            }
            TraceOpAggregateLazy::PushScope => Ok(TraceOpAggregate::PushScope),
            TraceOpAggregateLazy::PopScope => Ok(TraceOpAggregate::PopScope),
            TraceOpAggregateLazy::BranchStart { branch_end, } => {
                Ok(TraceOpAggregate::BranchStart {
                    branch_end: Owned::lazy_to_owned(branch_end)?,
                })
            }
            TraceOpAggregateLazy::BranchEnd { parent_branch_end, } => {
                Ok(TraceOpAggregate::BranchEnd {
                    parent_branch_end: Owned::lazy_to_owned(parent_branch_end)?,
                })
            }
            TraceOpAggregateLazy::Label { label, } => {
                Ok(TraceOpAggregate::Label {
                    label: Owned::lazy_to_owned(label)?,
                })
            }
            TraceOpAggregateLazy::Tag => Ok(TraceOpAggregate::Tag),
            TraceOpAggregateLazy::Metric { name,index, } => {
                Ok(TraceOpAggregate::Metric {
                    name: Owned::lazy_to_owned(name)?,
                    index: Owned::lazy_to_owned(index)?,
                })
            }
            TraceOpAggregateLazy::ChannelSend { channel, } => {
                Ok(TraceOpAggregate::ChannelSend {
                    channel: Owned::lazy_to_owned(channel)?,
                })
            }
            TraceOpAggregateLazy::ChannelReceive { channel, } => {
                Ok(TraceOpAggregate::ChannelReceive {
                    channel: Owned::lazy_to_owned(channel)?,
                })
            }
            TraceOpAggregateLazy::ChannelTransfer { from,to, } => {
                Ok(TraceOpAggregate::ChannelTransfer {
                    from: Owned::lazy_to_owned(from)?,
                    to: Owned::lazy_to_owned(to)?,
                })
            }
            TraceOpAggregateLazy::GlobalChannelSend { channel, } => {
                Ok(TraceOpAggregate::GlobalChannelSend {
                    channel: Owned::lazy_to_owned(channel)?,
                })
            }
            TraceOpAggregateLazy::GlobalChannelReceive { channel, } => {
                Ok(TraceOpAggregate::GlobalChannelReceive {
                    channel: Owned::lazy_to_owned(channel)?,
                })
            }
            TraceOpAggregateLazy::GlobalChannelTransfer { from,to, } => {
                Ok(TraceOpAggregate::GlobalChannelTransfer {
                    from: Owned::lazy_to_owned(from)?,
                    to: Owned::lazy_to_owned(to)?,
                })
            }
        }
    }
}

impl<'a> Copy for TraceOpAggregateLazy<'a> { }

impl<'a> core::fmt::Debug for TraceOpAggregateLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TraceOpAggregateLazy")
            .finish()
    }
}

impl<'a> PartialEq for TraceOpAggregateLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TraceOpAggregateLazy::CreateSource, TraceOpAggregateLazy::CreateSource) => true,
            (TraceOpAggregateLazy::DeleteSource, TraceOpAggregateLazy::DeleteSource) => true,
            (
                TraceOpAggregateLazy::Call {
                    source: self_source
                },
                TraceOpAggregateLazy::Call {
                    source: other_source
                },
            ) => {
                self_source == other_source
            }
            (TraceOpAggregateLazy::PushScope, TraceOpAggregateLazy::PushScope) => true,
            (TraceOpAggregateLazy::PopScope, TraceOpAggregateLazy::PopScope) => true,
            (
                TraceOpAggregateLazy::BranchStart {
                    branch_end: self_branch_end
                },
                TraceOpAggregateLazy::BranchStart {
                    branch_end: other_branch_end
                },
            ) => {
                self_branch_end == other_branch_end
            }
            (
                TraceOpAggregateLazy::BranchEnd {
                    parent_branch_end: self_parent_branch_end
                },
                TraceOpAggregateLazy::BranchEnd {
                    parent_branch_end: other_parent_branch_end
                },
            ) => {
                self_parent_branch_end == other_parent_branch_end
            }
            (
                TraceOpAggregateLazy::Label {
                    label: self_label
                },
                TraceOpAggregateLazy::Label {
                    label: other_label
                },
            ) => {
                self_label == other_label
            }
            (TraceOpAggregateLazy::Tag, TraceOpAggregateLazy::Tag) => true,
            (
                TraceOpAggregateLazy::Metric {
                    name: self_name, index: self_index
                },
                TraceOpAggregateLazy::Metric {
                    name: other_name, index: other_index
                },
            ) => {
                self_name == other_name
                    && self_index == other_index
            }
            (
                TraceOpAggregateLazy::ChannelSend {
                    channel: self_channel
                },
                TraceOpAggregateLazy::ChannelSend {
                    channel: other_channel
                },
            ) => {
                self_channel == other_channel
            }
            (
                TraceOpAggregateLazy::ChannelReceive {
                    channel: self_channel
                },
                TraceOpAggregateLazy::ChannelReceive {
                    channel: other_channel
                },
            ) => {
                self_channel == other_channel
            }
            (
                TraceOpAggregateLazy::ChannelTransfer {
                    from: self_from, to: self_to
                },
                TraceOpAggregateLazy::ChannelTransfer {
                    from: other_from, to: other_to
                },
            ) => {
                self_from == other_from
                    && self_to == other_to
            }
            (
                TraceOpAggregateLazy::GlobalChannelSend {
                    channel: self_channel
                },
                TraceOpAggregateLazy::GlobalChannelSend {
                    channel: other_channel
                },
            ) => {
                self_channel == other_channel
            }
            (
                TraceOpAggregateLazy::GlobalChannelReceive {
                    channel: self_channel
                },
                TraceOpAggregateLazy::GlobalChannelReceive {
                    channel: other_channel
                },
            ) => {
                self_channel == other_channel
            }
            (
                TraceOpAggregateLazy::GlobalChannelTransfer {
                    from: self_from, to: self_to
                },
                TraceOpAggregateLazy::GlobalChannelTransfer {
                    from: other_from, to: other_to
                },
            ) => {
                self_from == other_from
                    && self_to == other_to
            }
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MetricAggregate {
    pub count: u64,
    pub sum: i64,
    pub min: i64,
    pub max: i64,
}

pub struct MetricAggregateLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct MetricAggregateGen<> {
    pub count: u64,
    pub sum: i64,
    pub min: i64,
    pub max: i64,
}

impl<> Compatible<MetricAggregate> for MetricAggregateGen<> { }
impl<> Compatible<MetricAggregateGen<>> for MetricAggregate { }

impl<> BaseLen for MetricAggregateGen<> {
    const BASE_LEN: usize = 32;
}

impl<> Encode for MetricAggregateGen<> {
    fn scratch_len(&self) -> usize {
        self.count.scratch_len() + self.sum.scratch_len() + self.min.scratch_len() + self.max.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.count.encode(cursor);
        self.sum.encode(cursor);
        self.min.encode(cursor);
        self.max.encode(cursor);
    }
}

impl Owned for MetricAggregate {
    type Lazy<'a> = MetricAggregateLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for MetricAggregateLazy<'a> {
    type Owned = MetricAggregate;
}

impl<'a> Compatible<MetricAggregateLazy<'a>> for MetricAggregate { }
impl<'a> Compatible<MetricAggregate> for MetricAggregateLazy<'a> { }

impl<'a> MetricAggregateLazy<'a> {

    pub fn count(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn sum(&self) -> DecodeResult<i64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }

    pub fn min(&self) -> DecodeResult<i64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16))
    }

    pub fn max(&self) -> DecodeResult<i64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24))
    }
}

impl BaseLen for MetricAggregate {
    const BASE_LEN: usize = 32;
}

impl Encode for MetricAggregate {
    fn scratch_len(&self) -> usize {
        self.count.scratch_len() + self.sum.scratch_len() + self.min.scratch_len() + self.max.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.count.encode(cursor);
        self.sum.encode(cursor);
        self.min.encode(cursor);
        self.max.encode(cursor);
    }
}

impl<'a> Decode<'a> for MetricAggregate {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let count = Decode::decode(cursor)?;
        let sum = Decode::decode(cursor)?;
        let min = Decode::decode(cursor)?;
        let max = Decode::decode(cursor)?;

        Ok(MetricAggregate {
            count,
            sum,
            min,
            max,
        })
    }
}

impl<'a> BaseLen for MetricAggregateLazy<'a> {
    const BASE_LEN: usize = 32;
}

impl<'a> Encode for MetricAggregateLazy<'a> {
    fn scratch_len(&self) -> usize {
        let count: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let sum: i64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let min: i64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        let max: i64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24)).unwrap();
        count.scratch_len() + sum.scratch_len() + min.scratch_len() + max.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let count: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let sum: i64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        let min: i64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 16)).unwrap();
        let max: i64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 24)).unwrap();
        count.encode(cursor);
        sum.encode(cursor);
        min.encode(cursor);
        max.encode(cursor);
    }
}

impl<'a> Decode<'a> for MetricAggregateLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(MetricAggregateLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<MetricAggregateLazy<'a>> for MetricAggregate {
    type Error = DecodeError;

    fn try_from(other: MetricAggregateLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for MetricAggregateLazy<'a> { }

impl<'a> Clone for MetricAggregateLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for MetricAggregateLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MetricAggregateLazy")
            .finish()
    }
}

impl<'a> PartialEq for MetricAggregateLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.count().unwrap() == other.count().unwrap()
            && self.sum().unwrap() == other.sum().unwrap()&& self.min().unwrap() == other.min().unwrap()&& self.max().unwrap() == other.max().unwrap()
    }
}
