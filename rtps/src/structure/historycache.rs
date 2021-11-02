use super::{ChangeKind, Guid, InstanceHandle, SequenceNumber};

/// Container class used to temporarily store and manage sets of changes to data-objects.
/// On the Writer side it contains the history of the changes to data-objects made by the
/// Writer. It is not necessary that the full history of all changes ever made is maintained.
/// Rather what is needed is the partial history required to service existing and future
/// matched RTPS Reader endpoints. The partial history needed depends on the DDS
/// QoS and the state of the communications with the matched Reader endpoints.
/// On the Reader side it contains the history of the changes to data-objects made by the
/// matched RTPS Writer endpoints. It is not necessary that the full history of all changes
/// ever received is maintained. Rather what is needed is a partial history containing the
/// superposition of the changes received from the matched writers as needed to satisfy
/// the needs of the corresponding DDS DataReader. The rules for this superposition and
/// the amount of partial history required depend on the DDS QoS and the state of the
/// communication with the matched RTPS Writer endpoints.
pub trait HistoryCache {
    fn add_change(&mut self, change: CacheChange);

    fn remove_change(&mut self, change: &CacheChange);

    fn get_change(&self) -> &CacheChange;

    fn get_seq_num_min(&self) -> SequenceNumber;

    fn get_seq_num_max(&self) -> SequenceNumber;
}

/// Represents an individual change made to a data-object. Includes the creation,
/// modification, and deletion of data-objects.
pub struct CacheChange {
    kind: ChangeKind,
    writer_guid: Guid,
    instance_handle: InstanceHandle,
    sequence_number: SequenceNumber,
    data_value: Vec<Data>,
}

/// Represents the data that may be associated with a change made to a data-object.
pub struct Data;
