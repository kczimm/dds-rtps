//! A [`QoS`] (Quality of Service) is a set of characteristics that
//! controls some aspect of the behavior of the DDS Service.

use std::cmp::Ordering;

/// [`QoS`] (i.e., a list of QosPolicy objects) may be associated with all
/// Entity objects in the system such as [`Topic`], [`DataWriter`],
/// [`DataReader`], [`Publisher`], [`Subscriber`], and [`DomainParticipant`].
#[derive(Debug, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct QoS {
    user_data: Option<UserData>,
    topic_data: Option<TopicData>,
    group_data: Option<GroupData>,
    durability: Option<Durability>,
    durability_service: Option<DurabilityService>,
    presentation: Option<Presentation>,
    deadline: Option<Deadline>,
    latency_budget: Option<LatencyBudget>,
    ownership: Option<Ownership>,
    liveliness: Option<Liveliness>,
    time_based_filter: Option<TimeBasedFilter>,
    partition: Option<Partition>,
    reliability: Option<Reliability>,
    transport_priority: Option<TransportPriority>,
    lifespan: Option<Lifespan>,
    destination_order: Option<DestinationOrder>,
    history: Option<History>,
    resource_limits: Option<ResourceLimits>,
    entity_factory: Option<EntityFactory>,
    writer_data_lifecycle: Option<WriterDataLifecycle>,
    reader_data_lifecycle: Option<ReaderDataLifecycle>,
}

impl QoS {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn durability(&mut self, kind: DurabilityKind) -> &mut Self {
        self.durability = Some(Durability { kind });
        self
    }

    pub fn history(&mut self, kind: HistoryKind, depth: Option<usize>) -> &mut Self {
        self.history = Some(History { kind, depth });
        self
    }

    pub fn reliability(&mut self, kind: ReliabilityKind, max_blocking_time: Duration) -> &mut Self {
        self.reliability = Some(Reliability {
            kind,
            max_blocking_time,
        });
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

/// The purpose of this QoS is to allow the application to attach additional
/// information to the created Entity objects such that when
/// a remote application discovers their existence it can access that
/// information and use it for its own purposes. One possible use of this QoS is
/// to attach security credentials or some other information that can be used by
/// the remote application to authenticate the source. In combination with
/// operations such as ignore_participant, ignore_publication,
/// ignore_subscription, and ignore_topic these QoS can assist an application to
/// define and enforce its own security policies. The use of this QoS is not
/// limited to security, rather it offers a simple, yet flexible extensibility
/// mechanism.
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct UserData {
    value: Vec<u8>,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct TopicData {
    value: Vec<u8>,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct GroupData {
    datavalue: Vec<u8>,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Durability {
    kind: DurabilityKind,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct DurabilityService {
    service_cleanup_delay: Duration,
    history_kind: HistoryKind,
    history_depth: i32,
    max_samples: i32,
    max_instances: i32,
    max_samples_per_instance: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Presentation {
    access_scope: PresentationAccessScopeKind,
    coherent_access: bool,
    ordered_access: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Deadline {
    period: Duration,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct LatencyBudget {
    period: Duration,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Ownership {
    kind: OwnershipKind,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Liveliness {
    lease_duration: Duration,
    kind: LivelinessKind,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct TimeBasedFilter {
    minimum_separation: Duration,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Partition {
    name: String,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Reliability {
    kind: ReliabilityKind,
    max_blocking_time: Duration,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct TransportPriority {
    value: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Lifespan {
    duration: Duration,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct DestinationOrder {
    kind: DestinationOrderKind,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct History {
    kind: HistoryKind,
    depth: Option<usize>,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ResourceLimits {
    max_samples: i32,
    max_instances: i32,
    max_samples_per_instance: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct EntityFactory {
    autoenable_created_entities: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct WriterDataLifecycle {
    autodispose_unregistered_instances: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ReaderDataLifecycle {
    autopurge_nowriter_samples_delay: Duration,
    autopurge_disposed_samples_delay: Duration,
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
