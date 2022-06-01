use crate::structure::{
    historycache::{CacheChange, HistoryCache},
    participant::{self, Endpoint},
    Guid, Locator, ReliabilityKind, SequenceNumber, TopicKind,
};

use super::Duration;

pub mod stateful;
pub mod stateless;

pub struct Writer {
    guid: Guid,

    endpoint: Endpoint<participant::Writer>,

    push_mode: bool,
    heartbeat_period: Duration,
    nack_response_delay: Duration,
    nack_suppression_delay: Duration,
    last_change_sequence_number: SequenceNumber,
    data_max_size_serialized: i32,

    writer_cache: HistoryCache,
}

impl Writer {
    /// See 8.4.7.1.2 of the specification.
    pub fn new(
        guid: Guid,
        unicast_locator_list: Vec<Locator>,
        multicast_locator_list: Vec<Locator>,
        reliability_kind: ReliabilityKind,
        topic_kind: TopicKind,
        push_mode: bool,
        heartbeat_period: Duration,
        nack_response_delay: Duration,
        nack_suppression_duration: Duration,
    ) -> Self {
        let endpoint = Endpoint::new();
        Self {
            guid,
            endpoint: (),
            push_mode: (),
            heartbeat_period: (),
            nack_response_delay: (),
            nack_suppression_delay: (),
            last_change_sequence_number: (),
            data_max_size_serialized: (),
            writer_cache: (),
        }
    }

    pub fn new_change(&mut self) -> CacheChange {
        todo!()
    }
}
