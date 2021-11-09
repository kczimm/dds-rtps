//! The RTPS Simple Participant Discovery Protocol (SPDP) uses a simple approach
//! to announce and detect the presence of Participants in a domain.

use crate::{
    behavior::{writer::stateless::StatelessWriter, Duration},
    messages::Count,
    structure::{GuidPrefix, Locator, ProtocolVersion, VendorId},
};

pub struct SPDPdiscoveredParticipantData {
    participant_data: DiscoveredParticipantData,
    lease_duration: Duration,
}

struct DiscoveredParticipantData {
    proxy: ParticipantProxy,
    topic_data: ParticipantBuiltinTopicData,
}

struct ParticipantProxy {
    domain_id: DomainId,
    domain_tag: String,
    protocol_version: ProtocolVersion,
    guid_prefix: GuidPrefix,
    vendor_id: VendorId,
    expects_inline_qos: bool,
    available_builtin_endpoints: Vec<BuiltInEndpointKind>,
    builtin_endpoint_qos: BuiltinEndpointQos,
    meta_traffic_unicast_locator_list: Vec<Locator>,
    meta_traffic_multicast_locator_list: Vec<Locator>,
    default_multicast_locator_list: Vec<Locator>,
    default_unicast_locator_list: Vec<Locator>,
    manual_liveliness_count: Count,
}

type DomainId = u32;

struct BuiltInEndpointKind;

struct BuiltinEndpointQos;

struct ParticipantBuiltinTopicData {
    key: BuiltinTopicKey,
    user_data: UserDataQosPolicy,
}

struct BuiltinTopicKey;

struct UserDataQosPolicy;

struct SPDPbuiltinParticipantWriter {
    writer: StatelessWriter,
}
