use serde::Deserialize;

use super::*;

subset_declaration!(SubtypeDefinition SubtypeDefinitionNodeId {
    Integer(IntegerSubtypeDefinition),
    Floating(FloatingSubtypeDefinition),
    Physical(PhysicalSubtypeDefinition),
    Array(ArraySubtypeDefinition),
});

subset_declaration!(TypeDefinition TypeDefinitionNodeId {
    Array(ArrayTypeDefinition),
    Enumeration(EnumerationTypeDefinition),
});

subset_declaration!(AnonymousTypeDefinition AnonymousTypeDefinitionNodeId {
    Integer(IntegerTypeDefinition),
    Floating(FloatingTypeDefinition),
    Physical(PhysicalTypeDefinition),
    Array(ArrayTypeDefinition),
});

/// ```text
/// resolved_flag: bool
/// is_ref: bool
/// type_declarator: &anonymous_type_declaration
/// has_signal_flag: bool
/// signal_type_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerTypeDefinition {}

/// ```text
/// is_ref: bool
/// has_signal_flag: bool
/// range_constraint: &range_expression | &range_array_attribute
/// signal_type_flag: bool
/// type_declarator: &subtype_declaration
/// resolved_flag: bool
/// subtype_type_mark: &simple_name
/// parent_type: &integer_subtype_definition | &integer_type_definition
/// resolution_indication: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerSubtypeDefinition {
    pub range_constraint: NodeId<RangeExpression>,
}

/// ```text
/// resolved_flag: bool
/// has_signal_flag: bool
/// is_ref: bool
/// signal_type_flag: bool
/// type_declarator: &anonymous_type_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingTypeDefinition {}

/// ```text
/// range_constraint: &range_expression
/// type_declarator: &subtype_declaration
/// subtype_type_mark: &simple_name
/// is_ref: bool
/// resolution_indication: &simple_name
/// resolved_flag: bool
/// parent_type: &floating_subtype_definition | &floating_type_definition
/// has_signal_flag: bool
/// signal_type_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingSubtypeDefinition {
    pub range_constraint: NodeId<RangeExpression>,
}

/// ```text
/// has_signal_flag: bool
/// enumeration_literal_list: &[enumeration_literal]
/// range_constraint: &range_expression
/// is_ref: bool
/// resolved_flag: bool
/// signal_type_flag: bool
/// is_character_type: bool
/// only_characters_flag: bool
/// type_declarator: &type_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationTypeDefinition {
    pub enumeration_literal_list: Vec<NodeId<EnumerationLiteral>>,
}

/// ```text
/// has_signal_flag: bool
/// type_declarator: &subtype_declaration
/// range_constraint: &range_expression
/// resolution_indication: &simple_name
/// signal_type_flag: bool
/// subtype_type_mark: &simple_name
/// resolved_flag: bool
/// parent_type: &enumeration_type_definition
/// is_ref: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationSubtypeDefinition {}

/// ```text
/// type_declarator: &type_declaration
/// signal_type_flag: bool
/// resolved_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct WildcardTypeDefinition {}

/// ```text
/// has_signal_flag: bool
/// type_declarator: &anonymous_type_declaration
/// units: &[unit_declaration]
/// signal_type_flag: bool
/// resolved_flag: bool
/// is_ref: bool
/// end_has_reserved_id: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalTypeDefinition {
    pub units: Vec<NodeId<UnitDeclaration>>,
}

/// ```text
/// signal_type_flag: bool
/// type_declarator: &subtype_declaration
/// parent_type: &physical_type_definition | &physical_subtype_definition
/// has_signal_flag: bool
/// is_ref: bool
/// resolved_flag: bool
/// range_constraint: &range_expression
/// subtype_type_mark: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalSubtypeDefinition {
    pub parent_type: NodeId<PhysicalTypeDefinition>,
    pub range_constraint: NodeId<RangeExpression>,
}

/// ```text
/// index_constraint_flag: bool
/// element_subtype: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition
/// has_signal_flag: bool
/// resolved_flag: bool
/// signal_type_flag: bool
/// element_subtype_indication: &array_subtype_definition | &simple_name | &integer_subtype_definition
/// index_subtype_list: &[simple_name]
/// index_subtype_definition_list: &[simple_name]
/// constraint_state: "partially constrained" | "unconstrained"
/// type_declarator: &anonymous_type_declaration | &type_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayTypeDefinition {
    pub element_subtype: SubtypeDefinitionNodeId,
    #[serde(default)]
    pub index_constraint_list: Vec<NodeId<IntegerTypeDefinition>>,
}

/// ```text
/// resolution_indication: &simple_name
/// element_subtype: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition
/// parent_type: &array_type_definition | &array_subtype_definition
/// type_declarator: &subtype_declaration
/// index_subtype_list: &[enumeration_subtype_definition] | &[integer_subtype_definition] | &[simple_name]
/// constraint_state: "fully constrained" | "unconstrained"
/// signal_type_flag: bool
/// has_element_constraint_flag: bool
/// subtype_type_mark: &simple_name
/// index_constraint_list: &[enumeration_subtype_definition] | &[integer_subtype_definition] | &[simple_name]
/// has_signal_flag: bool
/// resolved_flag: bool
/// index_constraint_flag: bool
/// has_array_constraint_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArraySubtypeDefinition {
    pub element_subtype: SubtypeDefinitionNodeId,
    pub index_constraint_list: Vec<SubtypeDefinitionNodeId>,
}

/// ```text
/// signal_type_flag: bool
/// resolved_flag: bool
/// designated_type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &array_type_definition | &integer_subtype_definition | &access_type_definition
/// type_declarator: &type_declaration
/// designated_subtype_indication: &simple_name | &array_subtype_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTypeDefinition {}

/// ```text
/// file_type_mark: &simple_name
/// text_file_flag: bool
/// type_declarator: &type_declaration
/// signal_type_flag: bool
/// resolved_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FileTypeDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileDefinition {}
