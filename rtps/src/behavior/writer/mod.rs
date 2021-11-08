use crate::structure::{
    historycache::{CacheChange, HistoryCache},
    Guid, SequenceNumber,
};

use super::Duration;

#[derive(Debug)]
pub struct Writer<Cache> {
    guid: Guid,
    push_mode: bool,
    heartbeat_period: Duration,
    nack_response_delay: Duration,
    nack_suppression_delay: Duration,
    last_change_sequence_number: SequenceNumber,
    data_max_size_serialized: i32,
    cache: Cache,
}

impl<Cache> Writer<Cache> {
    pub fn guid() -> Guid {
        todo!()
    }
}

impl<Cache: Default + HistoryCache> Writer<Cache> {
    pub fn new() -> Self {
        Self {
            guid: Writer::<Cache>::guid(),
            push_mode: true,
            heartbeat_period: Duration::new(0, 0),
            nack_response_delay: Duration::new(0, 0),
            nack_suppression_delay: Duration::new(0, 0),
            last_change_sequence_number: SequenceNumber::default(),
            data_max_size_serialized: i32::MAX,
            cache: Cache::default(),
        }
    }

    pub fn new_change(&mut self) -> CacheChange {
        self.last_change_sequence_number.increment();

        todo!()
    }
}
