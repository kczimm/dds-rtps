//! The Real-Time Publish Subscribe (RTPS) protocol found its roots in
//! industrial automation and was in fact approved by the IEC as part of the
//! Real-Time Industrial Ethernet Suite IEC-PAS-62030. It is a field proven
//! technology that is currently deployed worldwide  in thousands of industrial
//! devices. RTPS was specifically developed to support the unique requirements
//! of data-distributions systems. As one of the application domains targeted by
//! DDS, the industrial automation community defined requirements for a standard
//! publish- subscribe wire-protocol that closely match those of DDS. As a
//! direct result, a close synergy exists between DDS and the RTPS
//! wire-protocol, both in terms  of the underlying behavioral architecture and
//! the features of RTPS

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![allow(dead_code)]

pub mod behavior;
pub mod discovery;
pub mod messages;
pub mod structure;
