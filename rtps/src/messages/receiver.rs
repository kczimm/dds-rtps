//! The interpretation and meaning of a Submessage within a Message may depend
//! on the previous Submessages contained within that same Message. Therefore,
//! the receiver of a Message must maintain state from previously deserialized
//! Submessages in the same Message. This state is modeled as the state of an
//! RTPS Receiver that is reset each time a new message is processed and
//! provides context for the interpretation of each Submessage.

use crate::{
    messages::{CHECKSUM_INVALID, MESSAGE_LENGTH_INVALID, TIME_INVALID},
    structure::{
        GuidPrefix, Locator, ProtocolVersion, VendorId, GUIDPREFIX_UNKNOWN, LOCATOR_INVALID,
        PROTOCOLVERSION, VENDORID_UNKNOWN,
    },
};

use super::{Checksum, MessageLength, ParameterId, Time};

struct Receiver {
    source_version: ProtocolVersion,
    source_vendor_id: VendorId,
    source_guid_prefix: GuidPrefix,
    dest_guid_prefix: GuidPrefix,
    unicast_reply_locator_list: Vec<Locator>,
    multicast_reply_locator_list: Vec<Locator>,
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
    pub fn new(dest_guid_prefix: GuidPrefix) -> Self {
        let locator = LOCATOR_INVALID;
        Self {
            source_version: PROTOCOLVERSION,
            source_vendor_id: VENDORID_UNKNOWN,
            source_guid_prefix: GUIDPREFIX_UNKNOWN,
            dest_guid_prefix,
            unicast_reply_locator_list: vec![locator],
            multicast_reply_locator_list: vec![locator],
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
