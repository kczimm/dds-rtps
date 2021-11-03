//! A QoS (Quality of Service) is a set of characteristics that
//! controls some aspect of the behavior of the DDS Service.

use std::cmp::Ordering;

pub enum QosPolicy {
    UserData {
        value: Vec<u8>,
    },
    TopicData {
        value: Vec<u8>,
    },
    GroupData {
        datavalue: Vec<u8>,
    },
    Durability {
        kind: DurabilityKind,
    },
    DurabilityService {
        service_cleanup_delay: Duration,
        history_kind: HistoryKind,
        history_depth: i32,
        max_samples: i32,
        max_instances: i32,
        max_samples_per_instance: i32,
    },
    Presentation {
        access_scope: PresentationAccessScopeKind,
        coherent_access: bool,
        ordered_access: bool,
    },
    Deadline {
        period: Duration,
    },
    LatencyBudget {
        period: Duration,
    },
    Ownership {
        kind: OwnershipKind,
    },
    Liveliness {
        lease_duration: Duration,
        kind: LivelinessKind,
    },
    TimeBasedFilter {
        minimum_separation: Duration,
    },
    Partition {
        name: String,
    },
    Reliability {
        kind: ReliabilityKind,
        max_blocking_time: Duration,
    },
    TransportPriority {
        value: i32,
    },
    Lifespan {
        duration: Duration,
    },
    DestinationOrder {
        kind: DestinationOrderKind,
    },
    History {
        kind: HistoryKind,
        depth: i32,
    },
    ResourceLimits {
        max_samples: i32,
        max_instances: i32,
        max_samples_per_instance: i32,
    },
    EntityFactory {
        autoenable_created_entities: bool,
    },
    WriterDataLifecycle {
        autodispose_unregistered_instances: bool,
    },
    ReaderDataLifecycle {
        autopurge_nowriter_samples_delay: Duration,
        autopurge_disposed_samples_delay: Duration,
    },
}

#[derive(Debug, PartialEq, Hash, Eq, Ord)]
pub enum DurabilityKind {
    Volatile,
    TransientLocal,
    Transient,
    Persistent,
}

impl DurabilityKind {
    /// The value offered is considered compatible with the value requested if
    /// and only if the inequality “offered kind >= requested
    /// kind” evaluates to ‘TRUE.’
    pub fn is_compatible(&self, offered: &Self) -> bool {
        offered > self
    }
}

impl PartialOrd for DurabilityKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Volatile => match other {
                Self::Volatile => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::TransientLocal => match other {
                Self::Volatile => Ordering::Greater,
                Self::TransientLocal => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::Transient => match other {
                Self::Persistent => Ordering::Less,
                Self::Transient => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::Persistent => match other {
                Self::Persistent => Ordering::Equal,
                _ => Ordering::Greater,
            },
        })
    }
}

pub type Duration = std::time::Duration;

#[derive(Debug, PartialEq, Hash, Eq, Ord)]
pub enum PresentationAccessScopeKind {
    Instance,
    Topic,
    Group,
}

impl PartialOrd for PresentationAccessScopeKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Instance => match other {
                Self::Instance => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::Topic => match other {
                Self::Instance => Ordering::Greater,
                Self::Topic => Ordering::Equal,
                Self::Group => Ordering::Less,
            },
            Self::Group => match other {
                Self::Group => Ordering::Equal,
                _ => Ordering::Greater,
            },
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum OwnershipKind {
    Shared,
    Exclusive { strength: usize },
}

#[derive(Debug, PartialEq, Hash, Eq, Ord)]
pub enum LivelinessKind {
    Automatic,
    ManualByParticipant,
    ManualByTopic,
}

impl PartialOrd for LivelinessKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Automatic => match other {
                Self::Automatic => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::ManualByParticipant => match other {
                Self::Automatic => Ordering::Greater,
                Self::ManualByParticipant => Ordering::Equal,
                Self::ManualByTopic => Ordering::Less,
            },
            Self::ManualByTopic => match other {
                Self::ManualByTopic => Ordering::Equal,
                _ => Ordering::Greater,
            },
        })
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Ord)]
pub enum ReliabilityKind {
    BestEffort,
    Reliable,
}

impl PartialOrd for ReliabilityKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::BestEffort => match other {
                Self::BestEffort => Ordering::Equal,
                Self::Reliable => Ordering::Less,
            },
            Self::Reliable => match other {
                Self::BestEffort => Ordering::Greater,
                Self::Reliable => Ordering::Equal,
            },
        })
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Ord)]
pub enum DestinationOrderKind {
    ByReceptionTimestamp,
    BySourceTimestamp,
}

impl PartialOrd for DestinationOrderKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::ByReceptionTimestamp => match other {
                Self::ByReceptionTimestamp => Ordering::Equal,
                Self::BySourceTimestamp => Ordering::Less,
            },
            Self::BySourceTimestamp => match other {
                Self::ByReceptionTimestamp => Ordering::Greater,
                Self::BySourceTimestamp => Ordering::Equal,
            },
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum HistoryKind {
    KeepAll,
    KeepLast,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_durabilitykind_partialord() {
        use DurabilityKind::*;
        assert!(Volatile < TransientLocal);
        assert!(TransientLocal < Transient);
        assert!(Transient < Persistent);
    }

    #[test]
    fn test_presentationaccessscopekind_partialord() {
        use PresentationAccessScopeKind::*;
        assert!(Instance < Topic);
        assert!(Topic < Group);
    }

    #[test]
    fn test_livelinesskind_partialord() {
        use LivelinessKind::*;
        assert!(Automatic < ManualByParticipant);
        assert!(ManualByParticipant < ManualByTopic);
    }

    #[test]
    fn test_reliabilitykind_partialord() {
        use ReliabilityKind::*;
        assert!(BestEffort < Reliable);
    }

    #[test]
    fn test_destinationorderkind_partialord() {
        use DestinationOrderKind::*;
        assert!(ByReceptionTimestamp < BySourceTimestamp);
    }
}
