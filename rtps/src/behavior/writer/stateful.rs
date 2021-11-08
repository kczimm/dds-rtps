use crate::structure::{historycache::CacheChange, Guid, Locator, SequenceNumber};

use super::Writer;

pub struct StatefulWriter {
    writer: Writer,
    matched_readers: Vec<ReaderProxy>,
}

pub struct ReaderProxy {
    remote_reader_guid: Guid,
    remote_group_guid: Guid,
    expects_inline_qos: bool,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,
    highest_sent_change_sn: Option<SequenceNumber>,
    requested_changes: Vec<SequenceNumber>,
    acknowledged_changes: Vec<SequenceNumber>,
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
