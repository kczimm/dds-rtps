//! The Messages module describes the overall structure and logical contents of
//! the messages that are exchanged between the RTPS Writer endpoints and RTPS
//! Reader endpoints. RTPS Messages are modular by design and can be easily
//! extended to support both standard protocol feature additions as well as
//! vendor-specific extensions.
//!
//! See the Section 8.3 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=37).

use crate::structure::{
    ChangeCount, EntityId, GuidPrefix, Locator, ProtocolVersion, SequenceNumber, VendorId,
};

mod receiver;

/// Enumeration used to identify the protocol.
type ProtocolId = [u8; 4];

const PROTOCOL_RTPS: ProtocolId = ['R' as _, 'T' as _, 'P' as _, 'S' as _];

/// Type used to specify a Submessage flag.
type SubmessageFlag = bool;

/// Enumeration used to identify the kind of Submessage.
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

/// Type used to hold a timestamp.
#[derive(Clone, Copy, Debug, Default, PartialEq, Hash, Eq)]
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

/// Type used to hold a count that is incremented monotonically, used to
/// identify message duplicates.
pub type Count = u32;

/// Type used to hold a checksum. Used to detect RTPS message corruption by the
/// underlying transport.
/// The following values are reserved by the protocol:
/// - CHECKSUM_INVALID.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct CheckSum;

/// Type used to hold the length of an RTPS Message.
pub type MessageLength = u32;

pub const MESSAGE_LENGTH_INVALID: MessageLength = 0;

/// Type used to uniquely identify a parameter in a parameter list.
/// Used extensively by the Discovery Module mainly to define QoS Parameters. A
/// range of values is reserved for protocol-defined parameters, while another
/// range can be used for vendor-defined parameters, see 8.3.5.9.
pub type ParameterId = i16;

/// Type used to hold fragment numbers.
pub type FragmentNumber = u32;

/// Type used to hold a digest value that uniquely identifies a group of
/// Entities belonging to the same [`Participant`].
pub type GroupDigest = [u8; 4];

/// Type used to hold an undefined 4-byte value. It is intended to be used in
/// future revisions of the specification.
pub type UExtension4 = [u8; 4];

/// Type used to hold an undefined 8-byte value. It is intended to be used in
/// future revisions of the specification.
pub type WExtension8 = [u8; 8];

/// The overall structure of an RTPS Message consists of a fixed-size leading
/// RTPS Header followed by a variable number of RTPS Submessage parts. Each
/// Submessage in turn consists of a SubmessageHeader and a variable
/// number of SubmessageElements. This is illustrated in Figure 8.8.
///
/// Each message sent by the RTPS protocol has a finite length. This length is
/// optionally sent in the RTPS HeaderExtension Submessage.
///
/// The length may also be sent by the underlying transport that carries the
/// RTPS message. In this case it may be omitted from the HeaderExtension. For
/// example, in the case of a packet-oriented transport (like UDP/IP), the
/// length of the message is already provided by the transport headers. In
/// contrast, a stream-oriented transport (like TCP) would need to include the
/// length in the RTPS HeaderExtension in order to identify the boundary of the
/// RTPS message.
pub struct Message {
    header: Header,
    header_extension: Option<HeaderExtension>,
    submessages: Vec<Submessage>,
}

/// The RTPS Header must appear at the beginning of every message.
///
/// The Header identifies the message as belonging to the RTPS protocol. The
/// Header identifies the version of the protocol and the vendor that sent  the
/// message.
pub struct Header {
    protocol: ProtocolId,
    version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: GuidPrefix,
}

/// The HeaderExtension may optionally appear immediately following the Header.
/// It extends the information provided in the Header without breaking
/// interoperability with earlier versions of the RTPS protocol.
///
/// The HeaderExtension submessage was introduced in RTPS version 2.5. Earlier
/// versions of the protocol (RTPS 2.4 and earlier) do not understand the
/// HeaderExtension submessage. However, since the HeaderExtension conforms to
/// the sub-message structure (see 8.3.3.3) versions of the protocol that do not
/// understand the HeaderExtension will treat it as “unknown messge kind”, skip
/// it, and continue processing the submessages that follow it, see 8.3.4.1.
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
    message_checksum: Option<CheckSum>,
    parameters: Option<ParameterList>,
}

pub struct ParameterList;

/// Each RTPS Message consists of a variable number of RTPS Submessage parts.
/// All RTPS Submessages feature the same identical structure.
///
/// All Submessages start with a SubmessageHeader part followed by a
/// concatenation of SubmessageElement parts. The SubmessageHeader identifies
/// the kind of Submessage and the optional elements within that Submessage.
pub struct Submessage {
    header: SubmessageHeader,
    elements: Vec<SubmessageElement>,
}

pub struct SubmessageHeader {
    submessage_id: SubmessageKind,
    flags: [SubmessageFlag; 8],
    submessage_length: u16,
}

/// Each RTPS message contains a variable number of RTPS Submessages. Each RTPS
/// Submessage in turn is built from a set of predefined atomic building blocks
/// called SubmessageElements.
pub enum SubmessageElement {
    ChangeCount {
        value: ChangeCount,
    },
    CheckSum {
        value: CheckSum,
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

pub struct KeyHashPrefix;
pub struct KeyHashSuffix;
pub struct Parameter {
    parameter_id: ParameterId,
    length: i16,
    value: Vec<u8>,
}
