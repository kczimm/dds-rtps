use crate::structure::{EntityId, Guid, Locator};

use super::Reader;

pub struct StatefulReader {
    reader: Reader,

    matched_writers: Vec<WriterProxy>,
}

impl StatefulReader {
    pub fn matched_writer_add(&self) {
        todo!()
    }

    pub fn matched_writer_remove(&self) {
        todo!()
    }

    pub fn matched_writer_lookup(&self) {
        todo!();
    }
}

pub struct WriterProxy {
    remote_writer_guid: Guid,
    remote_group_entity_id: EntityId,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,
    data_max_size_serialized: i32,
}

impl WriterProxy {
    pub fn available_changes_max(&self) {
        todo!()
    }

    pub fn irrelevant_change_set(&self) {
        todo!()
    }

    pub fn lost_changes_update(&self) {
        todo!()
    }

    pub fn missing_changes_update(&self) {
        todo!()
    }

    pub fn received_change_set(&self) {
        todo!()
    }

    pub fn missing_changes(&self) {
        todo!()
    }
}
