//! The DDS specification describes a Data-Centric Publish-Subscribe (DCPS)
//! model for distributed application communication and integration.
//!
//! See the DDS [specification](https://www.omg.org/spec/DDS/1.4/PDF) for more information.
//!
//! # Example Usage
//!
//! ```no_run
//! struct Shape {
//!     color: String,
//!     x: i32,
//!     y: i32,
//!     shapesize: i32,
//! }
//!
//! impl Keyed for Shape {
//!     type K = String;
//!
//!     fn get_key(&self) -> String {
//!         self.color.clone()
//!     }
//! }
//!
//! fn main() {
//!     let domain_id = 0;
//!     let dp = DomainParticipant::new(domain_id)?;
//!
//!     let topic_name = "Shape";
//!     let topic = dp.create_topic(topic_name, QoSPolicy::default())?;
//!     let publisher = dp.create_publisher(QoSPolicy::default())?;
//!     let mut writer = publisher.create_datawriter(topic.clone(), QoSPolicy::default())?;
//! }
//! ```

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![allow(dead_code)]

pub mod domain;
pub mod infrastructure;
pub mod publication;
pub mod qos;
pub mod subscription;
pub mod topic;
