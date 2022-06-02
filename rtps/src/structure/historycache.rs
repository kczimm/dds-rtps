use super::{ChangeKind, Guid, InstanceHandle, SequenceNumber};

pub struct HistoryCache {
    changes: Vec<CacheChange>,
}

impl HistoryCache {
    /// See section 8.2.2.1 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    /// See section 8.2.2.2 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    fn add_change(&mut self, change: CacheChange) {
        self.changes.push(change);
    }

    /// See section 8.2.2.3 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    fn remove_change(&mut self, change: &CacheChange) -> Option<CacheChange> {
        match self
            .changes
            .iter()
            .position(|c| c.sequence_number == change.sequence_number)
        {
            Some(index) => Some(self.changes.remove(index)),
            None => None,
        }
    }

    fn get_change(&self) -> Option<&CacheChange> {
        todo!()
    }

    /// See section 8.2.2.4 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    fn get_seq_num_min(&self) -> Option<SequenceNumber> {
        self.changes.iter().map(|c| c.sequence_number).min()
    }

    /// See section 8.2.2.5 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    fn get_seq_num_max(&self) -> Option<SequenceNumber> {
        self.changes.iter().map(|c| c.sequence_number).max()
    }
}

/// Represents an individual change made to a data-object. Includes the
/// creation, modification, and deletion of data-objects.
#[derive(Debug, PartialEq)]
pub struct CacheChange {
    kind: ChangeKind,
    writer_guid: Guid,
    instance_handle: InstanceHandle,
    pub(crate) sequence_number: SequenceNumber,
    data_value: Option<Data>,
    inline_qos: ParameterList,
}

#[derive(Debug, PartialEq)]
struct ParameterList;

/// Represents the data that may be associated with a change made to a
/// data-object.
#[derive(Debug, PartialEq)]
pub struct Data;
