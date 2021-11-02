//! This module describes the dynamic behavior of the RTPS entities. It
//! describes the valid sequences of message exchanges between RTPS Writer
//! endpoints and RTPS Reader endpoints and the timing constraints of those
//! messages.
//!
//! See Secion 8.4 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=72)

pub mod cache;
pub mod writer;

/// Type used to hold time differences.
/// Should have at least nano-second resolution.
pub type Duration = std::time::Duration;

/// Enumeration used to indicate the status of a
/// ChangeForReader. It can take the values:
/// - UNSENT
/// - UNACKNOWLEDGED,
/// - REQUESTED
/// - ACKNOWLEDGED
/// - UNDERWAY
pub enum ChangeForReaderStatusKind {
    Unsent,
    Unacknowledged,
    Requested,
    Acknowledged,
    Underway,
}

/// Enumeration used to indicate the status of a
/// ChangeFromWriter. It can take the values:
/// - NOT_AVAILABLE
/// - MISSING
/// - RECEIVED
/// - UNKNOWN
///
/// There are three sub-kinds of NOT_AVAILABLE:
/// - NA_FILTERED
/// - NA_REMOVED
/// - NA_UNSPECIFIED
pub enum ChangeFromWriterStatusKind {
    NotAvailableFiltered,
    NotAvailableRemoved,
    NotAvailableUnspecified,
    Missing,
    Received,
    Unknown,
}

/// Type used to represent the identity of a data-object whose changes in
/// value are communicated by the RTPS protocol.
pub struct InstanceHandle;

/// Type used to hold data exchanged between Participants. The most
/// notable use of this type is for the Writer Liveliness Protocol.
pub struct ParticipantMessageData;
