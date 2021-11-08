use crate::structure::{
    historycache::{CacheChange, HistoryCache},
    Entity, Guid, SequenceNumber,
};

use super::Duration;

pub mod stateful;
pub mod stateless;

pub struct Writer {
    guid: Guid,

    push_mode: bool,
    heartbeat_period: Duration,
    nack_response_delay: Duration,
    nack_suppression_delay: Duration,
    last_change_sequence_number: SequenceNumber,
    data_max_size_serialized: i32,

    writer_cache: Box<dyn HistoryCache>,
}

impl Entity for Writer {
    fn entity(&self) -> Guid {
        self.guid
    }
}

impl Writer {
    pub fn new_change(&mut self) -> CacheChange {
        todo!()
    }
}
