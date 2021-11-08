use crate::structure::{historycache::CacheChange, SequenceNumber};

use super::Writer;

pub struct StatelessWriter {
    writer: Writer,
    reader_locators: Vec<ReaderLocator>,
}

pub struct ReaderLocator {
    highest_sent_change_sn: Option<SequenceNumber>,
    requested_changes: Vec<SequenceNumber>,
}

impl ReaderLocator {
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
}
