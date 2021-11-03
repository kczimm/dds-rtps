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
}

#[derive(Debug)]
pub struct DataWriter<Data> {
    _phantom: std::marker::PhantomData<Data>,
}
