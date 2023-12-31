// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_CONNECTION_INFO: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_CONNECTION_INFO: i16 = 7;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_CONNECTION_INFO: [ConnectionInfo; 8] = [
  ConnectionInfo::UNKNOWN,
  ConnectionInfo::NEVER_CONNECTED,
  ConnectionInfo::CONNECTED,
  ConnectionInfo::DISCONNECTED,
  ConnectionInfo::DESTROYED,
  ConnectionInfo::CANCELLED,
  ConnectionInfo::FINISHED,
  ConnectionInfo::REMOTE_ERROR,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ConnectionInfo(pub i16);
#[allow(non_upper_case_globals)]
impl ConnectionInfo {
  pub const UNKNOWN: Self = Self(0);
  pub const NEVER_CONNECTED: Self = Self(1);
  pub const CONNECTED: Self = Self(2);
  pub const DISCONNECTED: Self = Self(3);
  pub const DESTROYED: Self = Self(4);
  pub const CANCELLED: Self = Self(5);
  pub const FINISHED: Self = Self(6);
  pub const REMOTE_ERROR: Self = Self(7);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 7;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::UNKNOWN,
    Self::NEVER_CONNECTED,
    Self::CONNECTED,
    Self::DISCONNECTED,
    Self::DESTROYED,
    Self::CANCELLED,
    Self::FINISHED,
    Self::REMOTE_ERROR,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::UNKNOWN => Some("UNKNOWN"),
      Self::NEVER_CONNECTED => Some("NEVER_CONNECTED"),
      Self::CONNECTED => Some("CONNECTED"),
      Self::DISCONNECTED => Some("DISCONNECTED"),
      Self::DESTROYED => Some("DESTROYED"),
      Self::CANCELLED => Some("CANCELLED"),
      Self::FINISHED => Some("FINISHED"),
      Self::REMOTE_ERROR => Some("REMOTE_ERROR"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ConnectionInfo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ConnectionInfo {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ConnectionInfo {
    type Output = ConnectionInfo;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ConnectionInfo {
  type Scalar = i16;
  #[inline]
  fn to_little_endian(self) -> i16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i16) -> Self {
    let b = i16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ConnectionInfo {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ConnectionInfo {}
pub enum EpicsPVConnectionInfoOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct EpicsPVConnectionInfo<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for EpicsPVConnectionInfo<'a> {
  type Inner = EpicsPVConnectionInfo<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> EpicsPVConnectionInfo<'a> {
  pub const VT_TIMESTAMP: flatbuffers::VOffsetT = 4;
  pub const VT_STATUS: flatbuffers::VOffsetT = 6;
  pub const VT_SOURCE_NAME: flatbuffers::VOffsetT = 8;
  pub const VT_SERVICE_ID: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    EpicsPVConnectionInfo { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args EpicsPVConnectionInfoArgs<'args>
  ) -> flatbuffers::WIPOffset<EpicsPVConnectionInfo<'bldr>> {
    let mut builder = EpicsPVConnectionInfoBuilder::new(_fbb);
    builder.add_timestamp(args.timestamp);
    if let Some(x) = args.service_id { builder.add_service_id(x); }
    if let Some(x) = args.source_name { builder.add_source_name(x); }
    builder.add_status(args.status);
    builder.finish()
  }


  #[inline]
  pub fn timestamp(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(EpicsPVConnectionInfo::VT_TIMESTAMP, Some(0)).unwrap()}
  }
  #[inline]
  pub fn status(&self) -> ConnectionInfo {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ConnectionInfo>(EpicsPVConnectionInfo::VT_STATUS, Some(ConnectionInfo::UNKNOWN)).unwrap()}
  }
  #[inline]
  pub fn source_name(&self) -> &'a str {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(EpicsPVConnectionInfo::VT_SOURCE_NAME, None).unwrap()}
  }
  #[inline]
  pub fn service_id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(EpicsPVConnectionInfo::VT_SERVICE_ID, None)}
  }
}

impl flatbuffers::Verifiable for EpicsPVConnectionInfo<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i64>("timestamp", Self::VT_TIMESTAMP, false)?
     .visit_field::<ConnectionInfo>("status", Self::VT_STATUS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("source_name", Self::VT_SOURCE_NAME, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("service_id", Self::VT_SERVICE_ID, false)?
     .finish();
    Ok(())
  }
}
pub struct EpicsPVConnectionInfoArgs<'a> {
    pub timestamp: i64,
    pub status: ConnectionInfo,
    pub source_name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub service_id: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for EpicsPVConnectionInfoArgs<'a> {
  #[inline]
  fn default() -> Self {
    EpicsPVConnectionInfoArgs {
      timestamp: 0,
      status: ConnectionInfo::UNKNOWN,
      source_name: None, // required field
      service_id: None,
    }
  }
}

pub struct EpicsPVConnectionInfoBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> EpicsPVConnectionInfoBuilder<'a, 'b> {
  #[inline]
  pub fn add_timestamp(&mut self, timestamp: i64) {
    self.fbb_.push_slot::<i64>(EpicsPVConnectionInfo::VT_TIMESTAMP, timestamp, 0);
  }
  #[inline]
  pub fn add_status(&mut self, status: ConnectionInfo) {
    self.fbb_.push_slot::<ConnectionInfo>(EpicsPVConnectionInfo::VT_STATUS, status, ConnectionInfo::UNKNOWN);
  }
  #[inline]
  pub fn add_source_name(&mut self, source_name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(EpicsPVConnectionInfo::VT_SOURCE_NAME, source_name);
  }
  #[inline]
  pub fn add_service_id(&mut self, service_id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(EpicsPVConnectionInfo::VT_SERVICE_ID, service_id);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EpicsPVConnectionInfoBuilder<'a, 'b> {
    let start = _fbb.start_table();
    EpicsPVConnectionInfoBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<EpicsPVConnectionInfo<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, EpicsPVConnectionInfo::VT_SOURCE_NAME,"source_name");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for EpicsPVConnectionInfo<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("EpicsPVConnectionInfo");
      ds.field("timestamp", &self.timestamp());
      ds.field("status", &self.status());
      ds.field("source_name", &self.source_name());
      ds.field("service_id", &self.service_id());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `EpicsPVConnectionInfo`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_epics_pvconnection_info_unchecked`.
pub fn root_as_epics_pvconnection_info(buf: &[u8]) -> Result<EpicsPVConnectionInfo, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<EpicsPVConnectionInfo>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `EpicsPVConnectionInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_epics_pvconnection_info_unchecked`.
pub fn size_prefixed_root_as_epics_pvconnection_info(buf: &[u8]) -> Result<EpicsPVConnectionInfo, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<EpicsPVConnectionInfo>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `EpicsPVConnectionInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_epics_pvconnection_info_unchecked`.
pub fn root_as_epics_pvconnection_info_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<EpicsPVConnectionInfo<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<EpicsPVConnectionInfo<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `EpicsPVConnectionInfo` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_epics_pvconnection_info_unchecked`.
pub fn size_prefixed_root_as_epics_pvconnection_info_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<EpicsPVConnectionInfo<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<EpicsPVConnectionInfo<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a EpicsPVConnectionInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `EpicsPVConnectionInfo`.
pub unsafe fn root_as_epics_pvconnection_info_unchecked(buf: &[u8]) -> EpicsPVConnectionInfo {
  flatbuffers::root_unchecked::<EpicsPVConnectionInfo>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed EpicsPVConnectionInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `EpicsPVConnectionInfo`.
pub unsafe fn size_prefixed_root_as_epics_pvconnection_info_unchecked(buf: &[u8]) -> EpicsPVConnectionInfo {
  flatbuffers::size_prefixed_root_unchecked::<EpicsPVConnectionInfo>(buf)
}
pub const EPICS_PVCONNECTION_INFO_IDENTIFIER: &str = "ep01";

#[inline]
pub fn epics_pvconnection_info_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, EPICS_PVCONNECTION_INFO_IDENTIFIER, false)
}

#[inline]
pub fn epics_pvconnection_info_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, EPICS_PVCONNECTION_INFO_IDENTIFIER, true)
}

#[inline]
pub fn finish_epics_pvconnection_info_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<EpicsPVConnectionInfo<'a>>) {
  fbb.finish(root, Some(EPICS_PVCONNECTION_INFO_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_epics_pvconnection_info_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<EpicsPVConnectionInfo<'a>>) {
  fbb.finish_size_prefixed(root, Some(EPICS_PVCONNECTION_INFO_IDENTIFIER));
}
