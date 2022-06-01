use crate::structure::{historycache::CacheChange, SequenceNumber};

use super::Writer;

pub struct StatelessWriter {
    writer: Writer,
    reader_locators: Vec<ReaderLocator>,
}

impl StatelessWriter {
    pub fn reader_locator_add(&mut self, _reader: ReaderLocator) {
        todo!()
    }

    pub fn reader_locator_remove(&mut self, _reader: &ReaderLocator) {
        todo!()
    }

    pub fn unsent_changes_reset(&mut self) {
        todo!()
    }
}

pub struct ReaderLocator {
    requested_changes: Vec<CacheChange>,
    highest_sent_change: Option<CacheChange>,
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
