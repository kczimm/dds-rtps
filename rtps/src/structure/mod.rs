//! Structure module.
//!
//! # Section 8.2.1 Overview
//!
//! RTPS entities are the protocol-level endpoints used by the
//! application-visible DDS entities in order to communicate with each other.
//!
//! Each RTPS Entity is in a one-to-one correspondence with a DDS Entity. The
//! HistoryCache forms the interface between the DDS Entities and their
//! corresponding RTPS Entities. For example, each write operation on a DDS
//! DataWriter adds a CacheChange to the HistoryCache of its corresponding RTPS
//! Writer. The RTPS Writer subsequently transfers the CacheChange to the
//! HistoryCache of all matching RTPS Readers. On the receiving side, the DDS
//! DataReader is notified by the RTPS Reader that a new CacheChange has arrived
//! in the HistoryCache, at which point the DDS DataReader may choose  to access
//! it using the DDS read or take API.
//!
//! See Section 8.2 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=21).

use std::cmp::Ordering;

pub mod historycache;
pub mod participant;

/// RTPS [`Entity`] represents the class of objects that are visible to other
/// RTPS Entities on the network. As such, RTPS [`Entity`] objects have a
/// globally-unique identifier ([`Guid`]) and can be referenced inside RTPS
/// messages.
pub trait Entity {
    fn entity(&self) -> Guid;
}

/// Type used to hold globally-unique RTPS-entity identifiers. These are
/// identifiers used to uniquely refer to each RTPS Entity in the system. Must
/// be possible to represent using 16 octets.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Guid {
    guid_prefix: GuidPrefix,
    entity_id: EntityId,
}

impl Guid {
    pub const fn new(guid_prefix: GuidPrefix, entity_id: EntityId) -> Self {
        Self {
            guid_prefix,
            entity_id,
        }
    }
}

/// Type used to hold the prefix of the globally-unique RTPS-entity identifiers.
/// The GUIDs of entities belonging to the same participant all have the same
/// prefix (see 8.2.4.3). Must be possible to represent using 12 octets.
pub type GuidPrefix = [u8; 12];

pub const GUIDPREFIX_UNKNOWN: GuidPrefix = [0; 12];

/// Type used to hold the suffix part of the globally-unique RTPS-entity
/// identifiers. The [`EntityId`] uniquely identifies an [`Entity`] within a
/// [`Participant`]. Must be possible to represent using 4 octets.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct EntityId {
    entity_key: [u8; 3],
    entity_kind: u8,
}

impl EntityId {
    #[must_use]
    pub const fn new(entity_key: [u8; 3], entity_kind: u8) -> Self {
        Self {
            entity_key,
            entity_kind,
        }
    }
}

pub const ENTITYID_UNKNOWN: EntityId = EntityId::new([0, 0, 0], 0);
pub const ENTITYID_PARTICIPANT: EntityId = EntityId::new([0, 0, 1], 0xc1);
pub const ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER: EntityId = EntityId::new([0, 1, 0], 0xc2);
pub const ENTITYID_SPDP_BUILTIN_PARTICIPANT_DETECTOR: EntityId = EntityId::new([0, 1, 0], 0xc7);
pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_ANNOUNCER: EntityId = EntityId::new([0, 0, 3], 0xc3);
pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_DETECTOR: EntityId = EntityId::new([0, 0, 3], 0xc7);
pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_ANNOUNCER: EntityId = EntityId::new([0, 0, 4], 0xc2);
pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_DETECTOR: EntityId = EntityId::new([0, 0, 4], 0xc7);
pub const ENTITYID_SEDP_BUILTIN_TOPICS_ANNOUNCER: EntityId = EntityId::new([0, 0, 2], 0xc2);
pub const ENTITYID_SEDP_BUILTIN_TOPICS_DETECTOR: EntityId = EntityId::new([0, 0, 2], 0xc7);
pub const ENTITYID_SEDP_BUILTIN_MESSAGE_WRITER: EntityId = EntityId::new([0, 2, 0], 0xc2);
pub const ENTITYID_SEDP_BUILTIN_MESSAGE_READER: EntityId = EntityId::new([0, 2, 0], 0xc7);

/// Type used to hold sequence numbers. Must be possible to represent using 64
/// bits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Hash, Eq, Ord)]
pub struct SequenceNumber {
    high: i32,
    low: u32,
}

impl SequenceNumber {
    #[must_use]
    pub fn value(&self) -> u64 {
        u64::from(self.low) + ((self.high as u64) << 32)
    }

    pub fn increment(&mut self) {
        self.low = if let Some(low) = self.low.checked_add(1) {
            low
        } else {
            self.high += 1;
            0
        };
    }
}

impl PartialOrd for SequenceNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

pub const SEQUENCENUMBER_UNKNOWN: SequenceNumber = SequenceNumber { high: -1, low: 0 };

/// Type used to represent the addressing information needed to send a message
/// to an RTPS Endpoint using one of the supported transports.
/// Should be able to hold a discriminator identifying the kind of transport, an
/// address, and a port number. It must be possible to represent the
/// discriminator and port number using 4 octets each, the address using 16
/// octets.

pub type Locator = std::net::SocketAddr;

// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
// pub struct Locator {
//     kind: LocatorKind,
//     port: LocatorPort,
//     address: LocatorAddress,
// }

// impl Locator {
//     pub fn new(kind: LocatorKind, port: LocatorPort, address: LocatorAddress)
// -> Self {         Self {
//             kind,
//             port,
//             address,
//         }
//     }
// }

// pub const LOCATOR_INVALID: Locator = Locator {
//     kind: LocatorKind::Invalid,
//     port: LOCATOR_PORT_INVALID,
//     address: LOCATOR_ADDRESS_INVALID,
// };

// impl TryFrom<Locator> for SocketAddr {
//     type Error = &'static str;

//     fn try_from(value: Locator) -> Result<Self, Self::Error> {
//         match value.kind {
//             LocatorKind::Invalid => Err("invalid locator kind"),
//             LocatorKind::Reserved => Err("reserved locator kind"),
//             _ => {
//                 let addr = Ipv6Addr::from(value.address);
//                 let ip = if let Some(addr) = addr.to_ipv4() {
//                     IpAddr::V4(addr)
//                 } else {
//                     IpAddr::from(addr)
//                 };
//                 Ok(SocketAddr::new(ip, value.port as _))
//             }
//         }
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
#[repr(i32)]
pub enum LocatorKind {
    Invalid = -1,
    Reserved = 0,
    UDPv4 = 1,
    UDPv6 = 2,
}

pub type LocatorPort = u32;

pub const LOCATOR_PORT_INVALID: LocatorPort = 0;

pub type LocatorAddress = [u8; 16];

pub const LOCATOR_ADDRESS_INVALID: LocatorAddress = [0; 16];

/// Enumeration used to distinguish whether a Topic has defined some fields
/// within to be used as the ‘key’ that identifies data-instances within the
/// Topic. See the DDS specification for more details on keys.
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
enum TopicKind {
    WithKey,
    NoKey,
}

/// Enumeration used to distinguish the kind of change that was made to a
/// data-object. Includes changes to the data or the instance state of the
/// data-object.
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum ChangeKind {
    Alive,
    AliveFiltered,
    NotAliveDisposed,
    NotAliveUnregistered,
}

/// Type used to hold a counter representing the number of [`HistoryCache`]
/// changes that belong to a certain category. For example, the number of
/// changes that have been filtered for an RTPS [`Reader`] [`Endpoint`].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ChangeCount {
    high: i32,
    low: u32,
}

impl ChangeCount {
    #[must_use]
    pub fn value(&self) -> u64 {
        u64::from(self.low) + ((self.high as u64) << 32)
    }
}

/// Enumeration used to indicate the level of the reliability used for
/// communications.
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
#[repr(i32)]
pub enum ReliabilityKind {
    BestEffort = 1,
    Reliable = 2,
}

/// Type used to represent the identity of a data-object whose changes in value
/// are communicated by the RTPS protocol.
#[derive(Debug, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct InstanceHandle;

/// Type used to represent the version of the RTPS protocol. The version is
/// composed of a major and a minor version number. See also 8.6.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ProtocolVersion {
    major: u8,
    minor: u8,
}

pub const PROTOCOLVERSION: ProtocolVersion = PROTOCOLVERSION_2_5;
pub const PROTOCOLVERSION_1_0: ProtocolVersion = ProtocolVersion { major: 1, minor: 0 };
pub const PROTOCOLVERSION_1_1: ProtocolVersion = ProtocolVersion { major: 1, minor: 1 };
pub const PROTOCOLVERSION_2_0: ProtocolVersion = ProtocolVersion { major: 2, minor: 0 };
pub const PROTOCOLVERSION_2_1: ProtocolVersion = ProtocolVersion { major: 2, minor: 1 };
pub const PROTOCOLVERSION_2_2: ProtocolVersion = ProtocolVersion { major: 2, minor: 2 };
pub const PROTOCOLVERSION_2_4: ProtocolVersion = ProtocolVersion { major: 2, minor: 4 };
pub const PROTOCOLVERSION_2_5: ProtocolVersion = ProtocolVersion { major: 2, minor: 5 };

impl Default for ProtocolVersion {
    fn default() -> Self {
        PROTOCOLVERSION
    }
}

/// Type used to represent the vendor of the service implementing the RTPS
/// protocol. The possible values for the vendorId are assigned by the OMG.
pub type VendorId = [u8; 2];

pub const VENDORID_UNKNOWN: VendorId = [0; 2];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_number_cmp() {
        let mut num = SequenceNumber::default();
        num.increment();
        assert!(num > SequenceNumber::default());
    }
}
