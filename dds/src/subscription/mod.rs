//! The Subscription Module contains the Subscriber, DataReader, ReadCondition,
//! and QueryCondition classes, as well as the SubscriberListener and
//! DataReaderListener interfaces, and more generally, all that is needed on the
//! subscription side.
//!
//! # Subscription View Example
//!
//! Code example from the sequence diagram in Section 2.2.6.2 of the [specification](https://www.omg.org/spec/DDS/1.4/PDF).
//!
//! ```no_run
//! struct Shape {
//!     color: String,
//!     x: i32,
//!     y: i32,
//!     shapesize: i32,
//! }
//!
//! async fn publish() {
//!     let domain_id = 0;
//!     let dp = DomainParticipant::new(domain_id)?;
//!
//!     let topic = dp.create_topic("Shape", QoS::topic_default())?;
//!     let mut subscriber = dp.create_subscriber(QoS::subscriber_default())?;
//!     let mut reader = subscriber.create_datareader::<Shape>(topic, QoS::datareader_default())?;
//!
//!     let mut sample = Shape::default();
//!     reader.read(&mut sample).await?;
//! }
//! ```

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Subscriber;
