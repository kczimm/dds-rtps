use crate::structure::{historycache::CacheChange, Guid, Locator, SequenceNumber};

use super::Writer;

pub struct StatefulWriter {
    writer: Writer,
    matched_readers: Vec<ReaderProxy>,
}

impl StatefulWriter {
    pub fn is_acked_by_all() -> bool {
        todo!()
    }

    pub fn matched_reader_add(&mut self, _reader: ReaderProxy) {
        todo!()
    }

    pub fn matched_reader_remove(&mut self, _reader: &ReaderProxy) {
        todo!()
    }
}

pub struct ReaderProxy {
    remote_reader_guid: Guid,
    remote_group_guid: Guid,
    expects_inline_qos: bool,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,
    highest_sent_change: Option<CacheChange>,
    requested_changes: Vec<CacheChange>,
    acknowledged_changes: Vec<CacheChange>,
}

impl ReaderProxy {
    pub fn acked_changes_set(&self) -> &[SequenceNumber] {
        todo!()
    }

    pub fn next_requested_change(&self) -> &CacheChange {
        todo!()
    }

    pub fn next_unsent_change(&self) -> &CacheChange {
        todo!()
    }

    pub fn requested_changes(&self) -> &[SequenceNumber] {
        todo!()
    }

    pub fn requested_changes_set(&self) -> &[SequenceNumber] {
        todo!()
    }

    pub fn unsent_changes(&self) -> &[SequenceNumber] {
        todo!()
    }

    pub fn unacked_changes(&self) -> &[SequenceNumber] {
        todo!()
    }
}
