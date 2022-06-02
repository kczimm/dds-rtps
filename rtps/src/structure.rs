//! Structure module.
//!
//! See Section 8.2 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=21).

use std::cmp::Ordering;

/// See Section 8.2.2 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=25)
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HistoryCache {
    changes: Vec<CacheChange>,
}

impl HistoryCache {
    /// See section 8.2.2.1 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    /// See section 8.2.2.2 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    pub fn add_change(&mut self, change: CacheChange) {
        self.changes.push(change);
    }

    /// See section 8.2.2.3 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    pub fn remove_change(&mut self, change: &CacheChange) -> Option<CacheChange> {
        match self
            .changes
            .iter()
            .position(|c| c.sequence_number == change.sequence_number)
        {
            Some(index) => Some(self.changes.remove(index)),
            None => None,
        }
    }

    /// See section 8.2.2.4 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    pub fn get_seq_num_min(&self) -> Option<SequenceNumber> {
        self.changes.iter().map(|c| c.sequence_number).min()
    }

    /// See section 8.2.2.5 of the [specification](https://www.omg.org/spec/DDSI-RTPS/).
    pub fn get_seq_num_max(&self) -> Option<SequenceNumber> {
        self.changes.iter().map(|c| c.sequence_number).max()
    }
}

/// See section 8.2.3 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=28).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct CacheChange {
    kind: ChangeKind,
    writer_guid: Guid,
    instance_handle: InstanceHandle,
    sequence_number: SequenceNumber,
    data_value: Option<Data>,
    inline_qos: ParameterList,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ParameterList;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Data;

/// See section 8.2.4 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=28).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Entity {
    guid: Guid,
}

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

pub type GuidPrefix = [u8; 12];

pub const GUIDPREFIX_UNKNOWN: GuidPrefix = [0; 12];

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

pub type Locator = std::net::SocketAddr;

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum TopicKind {
    WithKey,
    NoKey,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum ChangeKind {
    Alive,
    AliveFiltered,
    NotAliveDisposed,
    NotAliveUnregistered,
}

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
#[repr(i32)]
pub enum ReliabilityKind {
    BestEffort = 1,
    Reliable = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct InstanceHandle;

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

pub type VendorId = [u8; 2];

pub const VENDORID_UNKNOWN: VendorId = [0; 2];

/// See Section 8.2.5 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=30)
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Participant {
    entity: Entity,

    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    default_unicast_locator_list: Vec<Locator>,
    default_multicast_locator_list: Vec<Locator>,
    guid_prefix: GuidPrefix,

    publishers: Vec<Group>,
    subscribers: Vec<Group>,
}

/// See Section 8.2.6 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=31)
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Group {
    entity: Entity,

    endpoints: Vec<Endpoint>,
}

/// See Section 8.2.7 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=31)
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Endpoint {
    entity: Entity,

    topic_kind: TopicKind,
    reliability_level: ReliabilityKind,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,
    endpoint_group: EntityId,
}

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
