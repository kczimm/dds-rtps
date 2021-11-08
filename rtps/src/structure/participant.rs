use std::net::SocketAddr;

use super::{
    Entity, EntityId, Guid, GuidPrefix, Locator, ProtocolVersion, ReliabilityKind, TopicKind,
    VendorId,
};

/// RTPS [`Participant`] is the container of RTPS [`Group`] entities which
/// contain [`Endpoint`] entities. The RTPS [`Participant`] maps to a DDS
/// DomainParticipant.
#[derive(Debug)]
pub struct Participant {
    guid: Guid,

    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    default_unicast_locator_list: Vec<Locator>,
    default_multicast_locator_list: Vec<Locator>,
    guid_prefix: GuidPrefix,

    publishers: Vec<Publisher>,
    subscribers: Vec<Subscriber>,
}

impl Entity for Participant {
    fn entity(&self) -> Guid {
        self.guid
    }
}

#[derive(Debug)]
pub struct Group<Kind> {
    guid: Guid,

    endpoints: Vec<Endpoint<Kind>>,
}

/// Writer
#[derive(Debug)]
pub struct Writer;

#[derive(Debug)]
pub struct Reader;

pub type Publisher = Group<Writer>;
pub type Subscriber = Group<Reader>;

impl<Kind> Entity for Group<Kind> {
    fn entity(&self) -> Guid {
        self.guid
    }
}

#[derive(Debug)]
pub struct Endpoint<Kind> {
    guid: Guid,

    topic_kind: TopicKind,
    reliability_level: ReliabilityKind,
    unicast_locator_list: Vec<SocketAddr>,
    multicast_locator_list: Vec<SocketAddr>,
    endpoint_group: EntityId,

    _type: std::marker::PhantomData<Kind>,
}

impl<Kind> Entity for Endpoint<Kind> {
    fn entity(&self) -> Guid {
        self.guid
    }
}

impl Endpoint<Writer> {}
impl Endpoint<Reader> {}
