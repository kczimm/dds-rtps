//! The interpretation and meaning of a Submessage within a Message may depend on the previous Submessages
//! contained within that same Message. Therefore, the receiver of a Message must maintain state from previously
//! deserialized Submessages in the same Message. This state is modeled as the state of an RTPS Receiver that is
//! reset each time a new message is processed and provides context for the interpretation of each Submessage.

use crate::structure::{GuidPrefix, Locator, ProtocolVersion, VendorId};

use super::{CheckSum, MessageLength, ParameterId, Time};

struct Receiver {
    source_version: ProtocolVersion,
    source_vendor_id: VendorId,
    source_guid_prefix: GuidPrefix,
    unicast_reply_locator_list: Locator,
    multicast_reply_locator_list: Locator,
    have_timestamp: bool,
    timestamp: Time,
    message_length: MessageLength,
    message_checksum: CheckSum,
    rtps_send_timestamp: Time,
    rtps_reception_timestamp: Time,
    clock_skew_detected: bool,
    parameters: Vec<ParameterId>,
}
