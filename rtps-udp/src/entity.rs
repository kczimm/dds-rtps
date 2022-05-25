type OctetArray3 = [u8; 3];

#[repr(u8)]
enum EntityKind {
    BuiltInUnknown = 0xc0,
    BuiltInParticipant = 0xc1,
    BuiltInWriterWithKey = 0xc2,
    BuiltInWriterNoKey = 0xc3,
    BuiltInReaderNoKey = 0xc4,
    BuiltInReaderWithKey = 0xc7,
    BuiltInWriterGroup = 0xc8,
    BuiltInReaderGroup = 0xc9,

    UserDefinedUnknown = 0x00,
    // UserDefinedParticipant, N/A
    UserDefinedWriterWithKey = 0x02,
    UserDefinedWriterNoKey = 0x03,
    UserDefinedReaderNoKey = 0x04,
    UserDefinedReaderWithKey = 0x07,
    UserDefinedWriterGroup = 0x08,
    UserDefinedReaderGroup = 0x09,

    // All VendorSpecific codes inferred from Table 9.1 and Section 9.3.1.2
    VendorSpecificUnknown = 0x80,
    // VendorSpecificParticipant, N/A
    VendorSpecificWriterWithKey = 0x82,
    VendorSpecificWriterNoKey = 0x83,
    VendorSpecificReaderNoKey = 0x84,
    VendorSpecificReaderWithKey = 0x87,
    VendorSpecificWriterGroup = 0x88,
    VendorSpecificReaderGroup = 0x89,
}

struct EntityId {
    entity_key: OctetArray3,
    entity_kind: EntityKind,
}

const ENTITYID_UNKNOWN: EntityId = EntityId {
    entity_key: [0; 3],
    entity_kind: EntityKind::UserDefinedUnknown,
}; // see 9.3.1.2

const ENTITYID_PARTICIPANT: EntityId = EntityId {
    entity_key: [0, 0, 1],
    entity_kind: EntityKind::BuiltInParticipant,
};

type GuidPrefix = [u8; 12];
const GUIDPREFIX_UNKNOWN: GuidPrefix = [0; 12];

#[cfg(test)]
mod tests {}
