//! Messages module.
//!
//! See the Section 8.3 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=37).

use std::net::{Ipv4Addr, SocketAddr};

use crate::structure::{
    ChangeCount, EntityId, GuidPrefix, Locator, ProtocolVersion, SequenceNumber, VendorId,
    GUIDPREFIX_UNKNOWN, PROTOCOLVERSION, VENDORID_UNKNOWN,
};

type SubmessageFlag = bool;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
#[repr(u8)]
enum SubmessageKind {
    RtpsHe = 0x00,
    Pad = 0x01,
    AckNack = 0x06,
    Heartbeat = 0x07,
    Gap = 0x08,
    InfoTimestamp = 0x09,
    InfoSource = 0x0c,
    InfoReplyIp4 = 0x0d,
    InfoDestination = 0x0e,
    InfoReply = 0x0f,
    NackFrag = 0x12,
    HeartbeatFrag = 0x13,
    Data = 0x15,
    DataFrag = 0x16,
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct Time {
    seconds: u32,
    fraction: u32,
}

impl Time {
    #[must_use]
    pub const fn new(seconds: u32, fraction: u32) -> Time {
        Self { seconds, fraction }
    }
}

pub const TIME_ZERO: Time = Time::new(0, 0);
pub const TIME_INVALID: Time = Time::new(0xffffffff, 0xffffffff);
pub const TIME_INFINITE: Time = Time::new(0xffffffff, 0xfffffffe);

pub type Count = u32;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct Checksum;

pub const CHECKSUM_INVALID: Checksum = Checksum;

pub type MessageLength = u32;

pub const MESSAGE_LENGTH_INVALID: MessageLength = 0;

pub type ParameterId = i16;

pub type FragmentNumber = u32;

pub type GroupDigest = [u8; 4];

pub type UExtension4 = [u8; 4];

pub type WExtension8 = [u8; 8];

/// See Section 8.3.3 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=39)
#[derive(Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct Message {
    header: Header,
    header_extension: Option<HeaderExtension>,
    submessages: Vec<Submessage>,
}

/// See Section 8.3.4 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=44)
struct Receiver {
    source_version: ProtocolVersion,
    source_vendor_id: VendorId,
    source_guid_prefix: GuidPrefix,
    dest_guid_prefix: GuidPrefix,
    unicast_reply_locator_list: Vec<SocketAddr>,
    multicast_reply_locator_list: Vec<SocketAddr>,
    have_timestamp: bool,
    timestamp: Time,
    message_length: MessageLength,
    message_checksum: Checksum,
    rtps_send_timestamp: Time,
    rtps_reception_timestamp: Time,
    clock_skew_detected: bool,
    parameters: Vec<ParameterId>,
}

impl Receiver {
    pub fn new(dest_guid_prefix: GuidPrefix, addr: SocketAddr) -> Self {
        let unicast_locator = addr;
        let multicast_locator =
            SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        Self {
            source_version: PROTOCOLVERSION,
            source_vendor_id: VENDORID_UNKNOWN,
            source_guid_prefix: GUIDPREFIX_UNKNOWN,
            dest_guid_prefix,
            unicast_reply_locator_list: vec![unicast_locator],
            multicast_reply_locator_list: vec![multicast_locator],
            have_timestamp: false,
            timestamp: TIME_INVALID,
            message_length: MESSAGE_LENGTH_INVALID,
            message_checksum: CHECKSUM_INVALID,
            rtps_send_timestamp: TIME_INVALID,
            rtps_reception_timestamp: TIME_INVALID,
            clock_skew_detected: false,
            parameters: vec![],
        }
    }
}

/// See Section 8.3.3.1 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=40).
/// See also 8.3.6 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=52).
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct Header {
    protocol: ProtocolId,
    version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: GuidPrefix,
}

/// See Section 8.3.3.1.1 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=40)
type ProtocolId = [u8; 4];

const PROTOCOL_RTPS: ProtocolId = [b'R' as _, b'T' as _, b'P' as _, b'S' as _];

/// See Section 8.3.3.2 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=40)
/// See also 8.3.7 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=53).
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct HeaderExtension {
    endianness_flag: SubmessageFlag,
    length_flag: SubmessageFlag,
    timestamp_flag: SubmessageFlag,
    uextension_flag: SubmessageFlag,
    wextension_flag: SubmessageFlag,
    checksum_flags: [SubmessageFlag; 2],
    parameters_flag: SubmessageFlag,
    message_length: Option<MessageLength>,
    rtps_send_timestamp: Option<Time>,
    uextention4: Option<UExtension4>,
    wextention8: Option<WExtension8>,
    message_checksum: Option<Checksum>,
    parameters: Option<ParameterList>,
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct ParameterList;

/// See Section 8.3.3.3 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=43).
/// See also 8.3.8 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=53).
#[derive(Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct Submessage {
    header: SubmessageHeader,
    elements: Vec<SubmessageElement>,
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct SubmessageHeader {
    submessage_id: SubmessageKind,
    flags: [SubmessageFlag; 8],
    submessage_length: u16,
}

/// See Section 8.3.5 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=46).
#[derive(Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub enum SubmessageElement {
    ChangeCount {
        value: ChangeCount,
    },
    CheckSum {
        value: Checksum,
    },
    Count {
        value: Count,
    },
    EntityId {
        value: EntityId,
    },
    FragmentNumber {
        value: FragmentNumber,
    },
    FragmentNumberSet {
        base: FragmentNumber,
        set: Vec<FragmentNumber>,
    },
    GroupDigest {
        value: GroupDigest,
    },
    GuidPrefix {
        value: GuidPrefix,
    },
    KeyHashPrefix {
        value: KeyHashPrefix,
    },
    KeyHashSuffix {
        value: KeyHashSuffix,
    },
    LocatorList {
        value: Vec<Locator>,
    },
    MessageLength {
        value: MessageLength,
    },
    ParameterList {
        parameter: Parameter,
    },
    ProtocolVersion {
        value: ProtocolVersion,
    },
    SequenceNumber {
        value: SequenceNumber,
    },
    SequenceNumberSet {
        base: SequenceNumber,
        set: Vec<SequenceNumber>,
    },
    SerializedData {
        value: Vec<u8>,
    },
    StatusInfo {
        value: [SubmessageFlag; 32],
    },
    TimeStamp {
        value: Time,
    },
    VendorId {
        value: VendorId,
    },
    UExtension4 {
        value: UExtension4,
    },
    WExtension8 {
        value: WExtension8,
    },
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct KeyHashPrefix;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct KeyHashSuffix;

#[derive(Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct Parameter {
    parameter_id: ParameterId,
    length: i16,
    value: Vec<u8>,
}
