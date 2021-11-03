//! The DDS specification describes a Data-Centric Publish-Subscribe (DCPS)
//! model for distributed application communication and integration.
//!
//! See the DDS [specification](https://www.omg.org/spec/DDS/1.4/PDF) for more information.

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![allow(dead_code)]

pub mod domain;
pub mod infrastructure;
pub mod publication;
pub mod subscription;
pub mod topic;
