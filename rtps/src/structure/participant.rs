use crate::behavior::cache::VecCache;

use super::{
    historycache::HistoryCache, EntityId, Guid, Locator, ProtocolVersion, ReliabilityKind,
    TopicKind,
};

/// RTPS Participant is the container of RTPS Group entities which contain
/// Endpoint entities. The RTPS Participant maps to a DDS DomainParticipant.
#[derive(Debug)]
pub struct Participant<Cache = VecCache>
where
    Cache: HistoryCache,
{
    guid: Guid,
    protocol_version: ProtocolVersion,
    default_unicast_locator_list: Vec<Locator>,
    default_multicast_locator_list: Vec<Locator>,

    publishers: Group<Writer, Cache>,
    subscribers: Group<Reader, Cache>,
}

#[derive(Debug)]
pub struct Writer;

#[derive(Debug)]
pub struct Reader;

#[derive(Debug)]
pub struct Group<Type, Cache: HistoryCache> {
    guid: Guid,
    endpoints: Vec<Endpoint<Type, Cache>>,
}

#[derive(Debug)]
pub struct Endpoint<Type, Cache: HistoryCache> {
    guid: Guid,
    topic_kind: TopicKind,
    reliability_level: ReliabilityKind,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,
    endpoint_group: EntityId,
    _type: std::marker::PhantomData<Type>,

    cache: Cache,
}

impl<Cache: HistoryCache> Endpoint<Writer, Cache> {
    pub fn new_change(&mut self) {
        todo!()
    }
}
