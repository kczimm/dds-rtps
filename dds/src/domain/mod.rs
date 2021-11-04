//! The Domain Module contains the DomainParticipant class that acts as an
//! entry-point of the Service and acts as a factory for many of the classes.
//! The DomainParticipant also acts as a container for the other objects that
//! make up the Service.

use crate::{
    publication::Publisher,
    subscription::Subscriber,
    topic::{ContentFilteredTopic, MultiTopic, Topic},
};

#[derive(Debug)]
pub struct DomainParticipantFactory;

impl DomainParticipantFactory {
    pub fn create_participant() -> DomainParticipant {
        todo!()
    }

    pub fn delete_participant() {
        todo!()
    }

    pub fn lookup_particant() {
        todo!()
    }

    pub fn get_instance() {
        todo!()
    }

    pub fn get_qos() {
        todo!()
    }

    pub fn set_qos() {
        todo!()
    }
}

#[derive(Debug)]
pub struct DomainParticipant {
    domain_id: DomainId,
    publishers: Vec<Publisher>,
    subscribers: Vec<Subscriber>,
    topics: Vec<Topic>,
    content_filtered_topics: Vec<ContentFilteredTopic>,
    multi_topics: Vec<MultiTopic>,
}

impl DomainParticipant {
    pub fn ignore_participant() {
        todo!()
    }

    pub fn ignore_publication() {
        todo!()
    }

    pub fn ignore_subscription() {
        todo!()
    }

    pub fn create_publisher() -> Publisher {
        todo!()
    }

    pub fn delete_publisher() {
        todo!()
    }

    pub fn create_subscriber() -> Subscriber {
        todo!()
    }

    pub fn delete_subscriber() {
        todo!()
    }

    pub fn get_builtin_subscriber() {
        todo!()
    }

    pub fn lookup_topicdescription() {
        todo!()
    }

    pub fn create_multitopic() -> MultiTopic {
        todo!()
    }

    pub fn delete_multitopic() {
        todo!()
    }

    pub fn create_contentfilteredtopic() -> ContentFilteredTopic {
        todo!()
    }

    pub fn delete_contentfilteredtopic() {
        todo!()
    }

    pub fn assert_liveliness() {
        todo!()
    }

    pub fn delete_contained_entities() {
        todo!()
    }

    pub fn ignore_topic() {
        todo!()
    }

    pub fn create_topic() -> Topic {
        todo!()
    }

    pub fn delete_topic() {
        todo!()
    }

    pub fn find_topic() {
        todo!()
    }

    pub fn get_discovered_participants() {
        todo!()
    }

    pub fn get_discovered_participant_data() {
        todo!()
    }

    pub fn get_discovered_topics() {
        todo!()
    }

    pub fn get_discovered_topic_data() {
        todo!()
    }

    pub fn contains_entity() {
        todo!()
    }

    pub fn get_current_time() {
        todo!()
    }
}

pub type DomainId = u16;
