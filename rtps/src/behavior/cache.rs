use crate::structure::{
    historycache::{CacheChange, HistoryCache},
    SequenceNumber,
};

#[derive(Debug)]
pub struct VecCache {
    changes: Vec<CacheChange>,
}

impl Default for VecCache {
    fn default() -> Self {
        Self {
            changes: Vec::new(),
        }
    }
}

impl HistoryCache for VecCache {
    fn add_change(&mut self, change: CacheChange) {
        self.changes.push(change);
    }

    fn remove_change(&mut self, change: &CacheChange) {
        if let Some(i) = self.changes.iter().position(|c| c == change) {
            self.changes.remove(i);
        }
    }

    fn get_change(&self) -> Option<&CacheChange> {
        self.changes.first()
    }

    fn get_seq_num_min(&self) -> Option<SequenceNumber> {
        self.changes.iter().map(|c| c.sequence_number).min()
    }

    fn get_seq_num_max(&self) -> Option<SequenceNumber> {
        self.changes.iter().map(|c| c.sequence_number).max()
    }
}
