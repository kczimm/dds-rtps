//! Structure module. See Section 8.2 of the [specification](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=21).

use std::net::SocketAddr;

pub mod historycache;
pub mod participant;

/// RTPS Entity represents the class of objects that are visible to other RTPS
/// Entities on the network. As such, RTPS Entity objects have a globally-unique
/// identifier (GUID) and can be referenced inside RTPS messages.
pub trait Entity {
    fn entity(&self) -> Guid;
}

/// Type used to hold globally-unique RTPS-entity identifiers. These are
/// identifiers used to uniquely refer to each RTPS Entity in the system. Must
/// be possible to represent using 16 octets. The following values are reserved
/// by the protocol: GUID_UNKNOWN
pub type Guid = (GuidPrefix, EntityId);

/// Type used to hold the prefix of the globally-unique RTPS-entity identifiers.
/// The GUIDs of entities belonging to the same participant all have the same
/// prefix (see 8.2.4.3). Must be possible to represent using 12 octets. The
/// following values are reserved by the protocol: GUIDPREFIX_UNKNOWN
pub type GuidPrefix = [u8; 12];

/// Type used to hold the suffix part of the globally-unique RTPS-entity
/// identifiers. The EntityId_t uniquely identifies an Entity within a
/// Participant. Must be possible to represent using 4 octets. The following
/// values are reserved by the protocol: ENTITYID_UNKNOWN Additional pre-defined
/// values are defined by the Discovery module in 8.5
pub type EntityId = [u8; 4];

/// Type used to hold sequence numbers. Must be possible to represent using 64
/// bits. The following values are reserved by the protocol:
/// SEQUENCENUMBER_UNKNOWN
pub type SequenceNumber = i64;

/// Type used to represent the addressing information needed to send a message
/// to an RTPS Endpoint using one of the supported transports.
/// Should be able to hold a discriminator identifying the kind of transport, an
/// address, and a port number. It must be possible to represent the
/// discriminator and port number using 4 octets each, the address using 16
/// octets. The following values are reserved by the protocol:
/// - LOCATOR_INVALID
/// - LOCATOR_KIND_INVALID
/// - LOCATOR_KIND_RESERVED
/// - LOCATOR_KIND_UDPv4
/// - LOCATOR_KIND_UDPv6
/// - LOCATOR_ADDRESS_INVALID
/// - LOCATOR_PORT_INVALID
#[derive(Debug)]
pub struct Locator {
    discrimiator: SocketAddr,
}

/// Enumeration used to distinguish whether a Topic has defined some fields
/// within to be used as the ‘key’ that identifies data-instances within the
/// Topic. See the DDS specification for more details on keys.
/// The following values are reserved by the protocol:
/// - NO_KEY
/// - WITH_KEY
#[derive(Debug)]
enum TopicKind {
    WithKey,
    NoKey,
}

/// Enumeration used to distinguish the kind of change that was made to a
/// data-object. Includes changes to the data or the instance state of the
/// data-object. It can take the values:
/// - ALIVE
/// - ALIVE_FILTERED
/// - NOT_ALIVE_DISPOSED
/// - NOT_ALIVE_UNREGISTERED
#[derive(Debug, PartialEq)]
pub enum ChangeKind {
    Alive,
    AliveFiltered,
    NotAliveDisposed,
    NotAliveUnregistered,
}

/// Type used to hold a counter representing the number of HistoryCache changes
/// that belong to a certain category. For example, the number of changes that
/// have been filtered for an RTPS Reader endpoint.
pub type ChangeCount = u64;

/// Enumeration used to indicate the level of the reliability used for
/// communications. It can take the values:
/// - BEST_EFFORT
/// - RELIABLE
#[derive(Debug)]
pub enum ReliabilityKind {
    BestEffort,
    Reliable,
}

/// Type used to represent the identity of a data-object whose changes in value
/// are communicated by the RTPS protocol.
#[derive(Debug, PartialEq)]
pub struct InstanceHandle;

/// Type used to represent the version of the RTPS protocol. The version is
/// composed of a major and a minor version number. See also 8.6.
/// The following values are reserved by the protocol:
/// - PROTOCOLVERSION
/// - PROTOCOLVERSION_1_0
/// - PROTOCOLVERSION_1_1
/// - PROTOCOLVERSION_2_0
/// - PROTOCOLVERSION_2_1
/// - PROTOCOLVERSION_2_2
/// - PROTOCOLVERSION_2_4
///
/// PROTOCOLVERSION is an alias for the most recent version, in this case
/// PROTOCOLVERSION_2_4
#[derive(Debug)]
pub struct ProtocolVersion {
    major: (),
    minor: (),
}

/// Type used to represent the vendor of the service implementing the RTPS
/// protocol. The possible values for the vendorId are assigned by the OMG.
/// The following values are reserved by the protocol:
/// - VENDORID_UNKNOWN
#[derive(Debug)]
pub struct VendorId;
