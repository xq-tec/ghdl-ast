use serde::Deserialize;

use super::*;

subset_declaration!(SubtypeDefinition SubtypeDefinitionNodeId {
    Integer(IntegerSubtypeDefinition),
    Floating(FloatingSubtypeDefinition),
    Physical(PhysicalSubtypeDefinition),
    Enumeration(EnumerationSubtypeDefinition),
    Array(ArraySubtypeDefinition),
    Record(RecordSubtypeDefinition),
    Access(AccessSubtypeDefinition),
    File(FileSubtypeDefinition),
});

subset_declaration!(TypeDefinition TypeDefinitionNodeId {
    Array(ArrayTypeDefinition),
    Enumeration(EnumerationTypeDefinition),
    Access(AccessTypeDefinition),
    File(FileTypeDefinition),
    Record(RecordTypeDefinition),
    Incomplete(IncompleteTypeDefinition),
    Wildcard(WildcardTypeDefinition),
    /// Error node used when GHDL could not form a type definition.
    ErrorNode(Error),
});

subset_declaration!(AnonymousTypeDefinition AnonymousTypeDefinitionNodeId {
    Integer(IntegerTypeDefinition),
    Floating(FloatingTypeDefinition),
    Physical(PhysicalTypeDefinition),
    Array(ArrayTypeDefinition),
});

subset_declaration!(TypeAndSubtypeDefinition TypeAndSubtypeDefinitionNodeId {
    AccessType(AccessTypeDefinition),
    IncompleteType(IncompleteTypeDefinition),
    FileType(FileTypeDefinition),
    RecordType(RecordTypeDefinition),
    ArrayType(ArrayTypeDefinition),
    ArraySubtype(ArraySubtypeDefinition),
    RecordSubtype(RecordSubtypeDefinition),
    AccessSubtype(AccessSubtypeDefinition),
    FileSubtype(FileSubtypeDefinition),
    PhysicalSubtype(PhysicalSubtypeDefinition),
    FloatingSubtype(FloatingSubtypeDefinition),
    IntegerSubtype(IntegerSubtypeDefinition),
    EnumerationSubtype(EnumerationSubtypeDefinition),
    EnumerationType(EnumerationTypeDefinition),
    IntegerType(IntegerTypeDefinition),
    FloatingType(FloatingTypeDefinition),
    PhysicalType(PhysicalTypeDefinition),
    WildcardType(WildcardTypeDefinition),
});

#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerSubtypeDefinition {
    pub range_constraint: RangeConstraintNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingSubtypeDefinition {
    pub range_constraint: RangeConstraintNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationTypeDefinition {
    pub enumeration_literal_list: Vec<NodeId<EnumerationLiteral>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationSubtypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct WildcardTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalTypeDefinition {
    pub units: Vec<NodeId<UnitDeclaration>>,
}

subset_declaration!(PhysicalTypeOrSubtype PhysicalTypeOrSubtypeNodeId {
    Type(PhysicalTypeDefinition),
    Subtype(PhysicalSubtypeDefinition),
});

#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalSubtypeDefinition {
    pub parent_type: PhysicalTypeOrSubtypeNodeId,
    pub range_constraint: RangeConstraintNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayTypeDefinition {
    pub element_subtype: SubtypeDefinitionNodeId,
    #[serde(default)]
    pub index_subtype_list: Vec<NameNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArraySubtypeDefinition {
    pub element_subtype: SubtypeDefinitionNodeId,
    pub index_subtype_list: Vec<SubtypeDefinitionNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordSubtypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessSubtypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncompleteTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileSubtypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordElementConstraint {
    pub identifier: Identifier,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectedTypeDeclaration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectedTypeBody {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForeignVectorTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordModeViewIndication {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayModeViewIndication {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScalarNatureDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordNatureDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayNatureDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArraySubnatureDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordResolution {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordElementResolution {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleModeViewElement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayModeViewElement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordModeViewElement {}

subset_declaration!(RangeConstraint RangeConstraintNodeId {
    Expression(RangeExpression),
    Attribute(Attribute),
});
