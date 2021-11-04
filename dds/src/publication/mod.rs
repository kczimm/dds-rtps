//! The Publication Module contains the Publisher and DataWriter classes as well
//! as the PublisherListener and DataWriterListener interfaces, and more
//! generally, all that is needed on the publication side.

use crate::{qos::QosPolicy, topic::Topic};

#[derive(Debug)]
pub struct Publisher;

impl Publisher {
    pub fn new() -> Self {
        todo!()
    }

    pub fn create_datawriter<D>(&mut self, _topic: &Topic, _qos: &QosPolicy) -> DataWriter<D> {
        todo!()
    }

    pub fn delete_datawriter(&mut self) {
        todo!()
    }

    pub fn lookup_datawriter(&self, _topic_name: &str) {
        todo!()
    }

    pub fn suspend_publications(&mut self) {
        todo!()
    }

    pub fn resume_publications(&mut self) {
        todo!()
    }

    pub fn begin_coherent_changes(&mut self) {
        todo!()
    }

    pub fn end_coherent_changes(&mut self) {
        todo!()
    }

    pub fn wait_for_acknowledgements(&mut self, _max_wait: std::time::Duration) {
        todo!()
    }

    pub fn get_participant(&self) {
        todo!()
    }

    pub fn delete_contained_entities(&mut self) {
        todo!()
    }

    pub fn set_default_datawriter_qos(&mut self, _qos_list: &Vec<QosPolicy>) {
        todo!()
    }

    pub fn get_default_datawriter_qos(&self) -> &Vec<QosPolicy> {
        todo!()
    }

    pub fn copy_from_topic_qos(&self) {
        todo!()
    }
}

#[derive(Debug)]
pub struct DataWriter<Foo> {
    _phantom: std::marker::PhantomData<Foo>,
}

impl<Foo> DataWriter<Foo> {
    pub fn register_instance(&mut self, _instance: Foo) {
        todo!()
    }

    pub fn register_instance_w_timestamp(
        &mut self,
        _instance: Foo,
        _timestamp: std::time::SystemTime,
    ) {
        todo!()
    }

    pub fn unregister_instance(&mut self, _instance: Foo) {
        todo!()
    }

    pub fn unregister_instance_w_timestamp(
        &mut self,
        _instance: Foo,
        _timestamp: std::time::SystemTime,
    ) {
        todo!()
    }

    pub fn get_key_value(&self, _key_holder: Foo) {
        todo!()
    }

    pub fn write(&self, _instance_data: Foo) {
        todo!()
    }

    pub fn dispose(&self, _instance: Foo) {
        todo!()
    }

    pub fn dispose_w_timestamp(&self, _instance: Foo, _timestamp: std::time::SystemTime) {
        todo!()
    }
}
