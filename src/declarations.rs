use super::*;

subset_declaration!(Declaration DeclarationNodeId {
    Attribute(AttributeDeclaration),
    Subtype(SubtypeDeclaration),
    Type(TypeDeclaration),
    AnonymousType(AnonymousTypeDeclaration),

    Subprogram(SubprogramDeclaration),
    SubprogramBody(SubprogramBody),

    Constant(ConstantDeclaration),
    Signal(SignalDeclaration),
    Variable(VariableDeclaration),

    NonObjectAlias(NonObjectAliasDeclaration),

    SuspendState(SuspendStateDeclaration),
});

/// ```text
/// chain: &component_declaration | &subtype_declaration | &attribute_declaration | &function_declaration | &anonymous_type_declaration | &type_declaration | &file_declaration | &variable_declaration | &procedure_declaration | &constant_declaration | &signal_declaration
/// visible_flag: bool
/// subtype_indication: &floating_subtype_definition | &array_subtype_definition | &enumeration_subtype_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition | &access_subtype_definition
/// type: &floating_subtype_definition | &array_subtype_definition | &enumeration_subtype_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition | &access_subtype_definition
/// identifier: "…"
/// is_ref: bool
/// parent: int
#[derive(Debug, Deserialize, Serialize)]
pub struct SubtypeDeclaration {
    pub identifier: Identifier,
    pub subtype_indication: SubtypeDefinitionNodeId,
}

/// ```text
/// chain: &function_declaration
/// type_definition: &physical_type_definition | &integer_type_definition | &floating_type_definition | &array_type_definition
/// identifier: "…"
/// parent: int
/// subtype_definition: &floating_subtype_definition | &array_subtype_definition | &physical_subtype_definition | &integer_subtype_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousTypeDeclaration {
    pub type_definition: AnonymousTypeDefinitionNodeId,
    pub subtype_definition: Option<SubtypeDefinitionNodeId>,
}

/// ```text
/// identifier: "…"
/// chain: &function_declaration | &procedure_declaration | &type_declaration
/// parent: int
/// visible_flag: bool
/// incomplete_type_declaration: &type_declaration
/// type_definition: &record_type_definition | &error | &enumeration_type_definition | &wildcard_type_definition | &access_type_definition | &array_type_definition | &file_type_definition | &incomplete_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeDeclaration {
    pub identifier: Identifier,
    pub type_definition: TypeDefinitionNodeId,
}

/// ```text
/// visible_flag: bool
/// type_mark: &simple_name
/// type: &floating_subtype_definition | &enumeration_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_type_definition | &physical_subtype_definition | &array_type_definition | &integer_subtype_definition
/// parent: int
/// identifier: "…"
/// chain: &function_declaration | &attribute_specification | &attribute_declaration | &signal_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeDeclaration {
    pub identifier: Identifier,
}

/// ```text
/// visible_flag: bool
/// mode: "out" | "in" | "inout"
/// has_class: bool
/// identifier: "…"
/// is_ref: bool
/// after_drivers_flag: bool
/// subtype_indication: &simple_name
/// has_mode: bool
/// has_identifier_list: bool
/// chain: &interface_variable_declaration | &interface_constant_declaration
/// type: &file_type_definition
/// parent: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceFileDeclaration {
    pub identifier: Identifier,
}

/// ```text
/// identifier: "…"
/// visible_flag: bool
/// chain: &unit_declaration
/// physical_literal: &integer_literal
/// parent: int
/// type: &physical_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct UnitDeclaration {
    pub physical_literal: PhysicalLiteralNodeId,
}

/// ```text
/// parent: int
/// subtype_indication: &simple_name | &array_subtype_definition
/// is_ref: bool
/// open_flag: bool
/// type: &physical_type_definition | &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &floating_type_definition | &record_subtype_definition | &enumeration_type_definition | &integer_type_definition | &access_type_definition | &array_type_definition | &integer_subtype_definition | &physical_subtype_definition
/// has_identifier_list: bool
/// chain: &interface_variable_declaration | &interface_signal_declaration | &interface_constant_declaration
/// has_class: bool
/// default_value: &character_literal | &aggregate | &string_literal8 | &integer_literal | &physical_int_literal | &floating_point_literal | &simple_name
/// has_mode: bool
/// after_drivers_flag: bool
/// visible_flag: bool
/// mode: "in"
/// identifier: "…"
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceConstantDeclaration {
    pub identifier: Option<Identifier>,

    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    // pub source_location: SourceLocation,
}

/// ```text
/// subtype_indication: &array_subtype_definition | &selected_name | &simple_name | &integer_subtype_definition
/// identifier: "…"
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition | &array_type_definition
/// chain: &component_declaration | &subtype_declaration | &function_declaration | &use_clause | &anonymous_type_declaration | &type_declaration | &variable_declaration | &object_alias_declaration | &procedure_declaration | &attribute_specification | &constant_declaration | &signal_declaration
/// deferred_declaration_flag: bool
/// visible_flag: bool
/// has_identifier_list: bool
/// elaborated_flag: bool
/// deferred_declaration: &constant_declaration
/// is_ref: bool
/// parent: int
/// default_value: &string_literal8 | &aggregate | &length_array_attribute | &character_literal | &simple_aggregate | &integer_literal | &physical_int_literal | &enumeration_literal | &concatenation_operator | &floating_point_literal | &simple_name | &attribute_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstantDeclaration {
    pub identifier: Identifier,

    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    // pub source_location: SourceLocation,
}

/// ```text
/// after_drivers_flag: bool
/// has_active_flag: bool
/// chain: &interface_signal_declaration | &interface_constant_declaration
/// has_class: bool
/// parent: int
/// subtype_indication: &array_subtype_definition | &enumeration_subtype_definition | &simple_name
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition | &array_type_definition
/// visible_flag: bool
/// default_value: &string_literal8 | &character_literal | &aggregate | &integer_literal | &physical_int_literal | &floating_point_literal | &simple_name
/// mode: "in" | "out" | "inout" | "linkage" | "buffer"
/// has_mode: bool
/// has_disconnect_flag: bool
/// identifier: "…"
/// has_identifier_list: bool
/// is_ref: bool
/// open_flag: bool
/// signal_kind: "bus"
/// guarded_signal_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceSignalDeclaration {
    pub identifier: Identifier,

    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,

    pub mode: Mode,
}

/// ```text
/// parent: int
/// after_drivers_flag: bool
/// chain: &attribute_implicit_declaration | &component_declaration | &function_declaration | &attribute_declaration | &configuration_specification | &subtype_declaration | &type_declaration | &disconnection_specification | &attribute_specification | &procedure_declaration | &object_alias_declaration | &constant_declaration | &signal_declaration
/// guarded_signal_flag: bool
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition
/// visible_flag: bool
/// has_disconnect_flag: bool
/// has_active_flag: bool
/// has_identifier_list: bool
/// default_value: &string_literal8 | &character_literal | &aggregate | &identity_operator | &selected_name | &simple_aggregate | &integer_literal | &physical_int_literal | &enumeration_literal | &floating_point_literal | &negation_operator | &simple_name
/// subtype_indication: &array_subtype_definition | &enumeration_subtype_definition | &selected_name | &simple_name | &integer_subtype_definition
/// signal_kind: "bus" | "register"
/// is_ref: bool
/// identifier: "…"
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SignalDeclaration {
    pub identifier: Identifier,

    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

/// ```text
/// interface_variable_declaration:
///
/// has_mode: bool
/// has_class: bool
/// after_drivers_flag: bool
/// default_value: &physical_int_literal | &character_literal | &floating_point_literal | &simple_name | &integer_literal
/// has_identifier_list: bool
/// parent: int
/// mode: "in" | "inout" | "out"
/// identifier: "…"
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &access_type_definition | &array_type_definition | &integer_subtype_definition | &physical_subtype_definition
/// is_ref: bool
/// chain: &interface_variable_declaration | &interface_file_declaration | &interface_signal_declaration | &interface_constant_declaration
/// visible_flag: bool
/// subtype_indication: &simple_name
/// ```
///
/// ```text
/// visible_flag: bool
/// has_identifier_list: bool
/// identifier: "…"
/// parent: int
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &access_type_definition | &integer_subtype_definition | &physical_subtype_definition | &access_subtype_definition
/// subtype_indication: &array_subtype_definition | &selected_name | &simple_name | &access_subtype_definition
/// chain: &subtype_declaration | &function_declaration | &anonymous_type_declaration | &type_declaration | &variable_declaration | &procedure_declaration | &object_alias_declaration | &file_declaration | &constant_declaration
/// shared_flag: bool
/// default_value: &division_operator | &character_literal | &multiplication_operator | &null_literal | &allocator_by_subtype | &enumeration_literal | &allocator_by_expression | &simple_name | &string_literal8 | &aggregate | &function_call | &integer_literal | &physical_int_literal | &floating_point_literal
/// is_ref: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct VariableDeclaration {
    pub identifier: Identifier,

    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,

    pub default_value: Option<ExpressionNodeId>,
}

// TODO
#[derive(Debug, Deserialize, Serialize)]
pub struct NonObjectAliasDeclaration {
    pub identifier: Identifier,
}

// TODO
#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    #[serde(default, rename = "type_marks_list")]
    pub type_marks: Vec<SubtypeDefinitionNodeId>,
}

/// ```text
/// suspend_state_chain: &suspend_state_statement
/// chain: &subtype_declaration | &function_declaration | &use_clause | &anonymous_type_declaration | &type_declaration | &variable_declaration | &procedure_declaration | &file_declaration | &constant_declaration
/// parent: int
/// suspend_state_last: &suspend_state_statement
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SuspendStateDeclaration {}

/// ```text
/// procedure_declaration:
///
/// wait_state: "unknown"
/// foreign_flag: bool
/// has_body: bool
/// chain: &function_declaration | &subtype_declaration | &attribute_declaration | &anonymous_type_declaration | &function_body | &type_declaration | &variable_declaration | &procedure_declaration | &attribute_specification | &file_declaration | &constant_declaration | &signal_declaration
/// has_parameter: bool
/// interface_declarations: &[interface_file_declaration] | &[interface_signal_declaration] | &[interface_constant_declaration]
/// pure_flag: bool
/// return_type_mark: &simple_name
/// subprogram_body: &function_body
/// implicit_definition: "IIR_PREDEFINED_BIT_NAND" | "IIR_PREDEFINED_BOOLEAN_XOR" | "IIR_PREDEFINED_NOW_FUNCTION" | "IIR_PREDEFINED_BIT_XNOR" | "IIR_PREDEFINED_INTEGER_LESS" | "IIR_PREDEFINED_FOREIGN_TEXTIO_READ_REAL" | "IIR_PREDEFINED_INTEGER_EXP" | "IIR_PREDEFINED_UNIVERSAL_R_I_MUL" | "IIR_PREDEFINED_ARRAY_SLA" | "IIR_PREDEFINED_ARRAY_LESS" | "IIR_PREDEFINED_BIT_XOR" | "IIR_PREDEFINED_PHYSICAL_LESS" | "IIR_PREDEFINED_PHYSICAL_GREATER_EQUAL" | "IIR_PREDEFINED_BOOLEAN_NOR" | "IIR_PREDEFINED_FLOATING_INEQUALITY" | "IIR_PREDEFINED_ARRAY_EQUALITY" | "IIR_PREDEFINED_TF_ARRAY_AND" | "IIR_PREDEFINED_REAL_PHYSICAL_MUL" | "IIR_PREDEFINED_FLOATING_LESS_EQUAL" | "IIR_PREDEFINED_ENUM_LESS_EQUAL" | "IIR_PREDEFINED_RECORD_INEQUALITY" | "IIR_PREDEFINED_TF_ARRAY_XOR" | "IIR_PREDEFINED_INTEGER_ABSOLUTE" | "IIR_PREDEFINED_ENUM_GREATER_EQUAL" | "IIR_PREDEFINED_INTEGER_PLUS" | "IIR_PREDEFINED_FLOATING_EXP" | "IIR_PREDEFINED_PHYSICAL_PHYSICAL_DIV" | "IIR_PREDEFINED_INTEGER_EQUALITY" | "IIR_PREDEFINED_INTEGER_GREATER" | "IIR_PREDEFINED_PHYSICAL_LESS_EQUAL" | "IIR_PREDEFINED_PHYSICAL_MINUS" | "IIR_PREDEFINED_BOOLEAN_NOT" | "IIR_PREDEFINED_PHYSICAL_NEGATION" | "IIR_PREDEFINED_BOOLEAN_AND" | "IIR_PREDEFINED_INTEGER_MOD" | "IIR_PREDEFINED_INTEGER_MUL" | "IIR_PREDEFINED_FLOATING_DIV" | "IIR_PREDEFINED_FLOATING_MUL" | "IIR_PREDEFINED_ARRAY_SRL" | "IIR_PREDEFINED_PHYSICAL_INEQUALITY" | "IIR_PREDEFINED_ENUM_EQUALITY" | "IIR_PREDEFINED_INTEGER_LESS_EQUAL" | "IIR_PREDEFINED_INTEGER_MINUS" | "IIR_PREDEFINED_FLOATING_NEGATION" | "IIR_PREDEFINED_INTEGER_IDENTITY" | "IIR_PREDEFINED_ARRAY_ROR" | "IIR_PREDEFINED_TF_ARRAY_NOT" | "IIR_PREDEFINED_ACCESS_INEQUALITY" | "IIR_PREDEFINED_RECORD_EQUALITY" | "IIR_PREDEFINED_PHYSICAL_GREATER" | "IIR_PREDEFINED_INTEGER_DIV" | "IIR_PREDEFINED_ARRAY_GREATER" | "IIR_PREDEFINED_INTEGER_NEGATION" | "IIR_PREDEFINED_FLOATING_GREATER" | "IIR_PREDEFINED_FLOATING_ABSOLUTE" | "IIR_PREDEFINED_BIT_OR" | "IIR_PREDEFINED_PHYSICAL_PLUS" | "IIR_PREDEFINED_BOOLEAN_OR" | "IIR_PREDEFINED_PHYSICAL_ABSOLUTE" | "IIR_PREDEFINED_PHYSICAL_REAL_MUL" | "IIR_PREDEFINED_ARRAY_ARRAY_CONCAT" | "IIR_PREDEFINED_ARRAY_ELEMENT_CONCAT" | "IIR_PREDEFINED_PHYSICAL_INTEGER_DIV" | "IIR_PREDEFINED_PHYSICAL_IDENTITY" | "IIR_PREDEFINED_BOOLEAN_NAND" | "IIR_PREDEFINED_FLOATING_GREATER_EQUAL" | "IIR_PREDEFINED_INTEGER_PHYSICAL_MUL" | "IIR_PREDEFINED_TF_ARRAY_OR" | "IIR_PREDEFINED_BIT_NOT" | "IIR_PREDEFINED_ARRAY_SLL" | "IIR_PREDEFINED_FLOATING_LESS" | "IIR_PREDEFINED_ARRAY_SRA" | "IIR_PREDEFINED_ENUM_LESS" | "IIR_PREDEFINED_ENDFILE" | "IIR_PREDEFINED_ACCESS_EQUALITY" | "IIR_PREDEFINED_FLOATING_MINUS" | "IIR_PREDEFINED_UNIVERSAL_R_I_DIV" | "IIR_PREDEFINED_FLOATING_IDENTITY" | "IIR_PREDEFINED_UNIVERSAL_I_R_MUL" | "IIR_PREDEFINED_ARRAY_LESS_EQUAL" | "IIR_PREDEFINED_TF_ARRAY_NOR" | "IIR_PREDEFINED_FLOATING_EQUALITY" | "IIR_PREDEFINED_PHYSICAL_REAL_DIV" | "IIR_PREDEFINED_PHYSICAL_INTEGER_MUL" | "IIR_PREDEFINED_ARRAY_GREATER_EQUAL" | "IIR_PREDEFINED_ELEMENT_ELEMENT_CONCAT" | "IIR_PREDEFINED_TF_ARRAY_NAND" | "IIR_PREDEFINED_BIT_NOR" | "IIR_PREDEFINED_BIT_AND" | "IIR_PREDEFINED_INTEGER_GREATER_EQUAL" | "IIR_PREDEFINED_INTEGER_REM" | "IIR_PREDEFINED_ARRAY_ROL" | "IIR_PREDEFINED_INTEGER_INEQUALITY" | "IIR_PREDEFINED_NONE" | "IIR_PREDEFINED_ELEMENT_ARRAY_CONCAT" | "IIR_PREDEFINED_TF_ARRAY_XNOR" | "IIR_PREDEFINED_BOOLEAN_XNOR" | "IIR_PREDEFINED_ARRAY_INEQUALITY" | "IIR_PREDEFINED_ENUM_GREATER" | "IIR_PREDEFINED_PHYSICAL_EQUALITY" | "IIR_PREDEFINED_FLOATING_PLUS" | "IIR_PREDEFINED_ENUM_INEQUALITY"
/// subprogram_hash: int
/// is_within_flag: bool
/// resolution_function_flag: bool
/// seen_flag: bool
/// parent: int
/// hide_implicit_flag: bool
/// all_sensitized_state: "???" | "no_signal"
/// subprogram_depth: int
/// return_type: &physical_type_definition | &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &floating_type_definition | &enumeration_type_definition | &integer_type_definition | &physical_subtype_definition | &array_type_definition | &integer_subtype_definition | &record_subtype_definition
/// elaborated_flag: bool
/// has_pure: bool
/// identifier: "…"
/// visible_flag: bool
/// overload_number: int
///
///
/// procedure_declaration:
///
/// subprogram_depth: int
/// has_body: bool
/// parent: int
/// visible_flag: bool
/// overload_number: int
/// wait_state: "true" | "unknown" | "false"
/// all_sensitized_state: "no_signal" | "???"
/// implicit_definition: "IIR_PREDEFINED_READ_LENGTH" | "IIR_PREDEFINED_FILE_CLOSE" | "IIR_PREDEFINED_WRITE" | "IIR_PREDEFINED_FOREIGN_TEXTIO_WRITE_REAL" | "IIR_PREDEFINED_DEALLOCATE" | "IIR_PREDEFINED_FILE_OPEN_STATUS" | "IIR_PREDEFINED_NONE" | "IIR_PREDEFINED_FOREIGN_UNTRUNCATED_TEXT_READ" | "IIR_PREDEFINED_FILE_OPEN" | "IIR_PREDEFINED_READ"
/// passive_flag: bool
/// interface_declarations: &[interface_file_declaration] | &[interface_variable_declaration] | &[interface_signal_declaration] | &[interface_constant_declaration]
/// elaborated_flag: bool
/// subprogram_hash: int
/// suspend_flag: bool
/// foreign_flag: bool
/// hide_implicit_flag: bool
/// has_parameter: bool
/// identifier: "…"
/// seen_flag: bool
/// subprogram_body: &procedure_body
/// is_within_flag: bool
/// purity_state: "pure" | "impure" | "maybe_impure" | "unknown"
/// chain: &subtype_declaration | &function_declaration | &procedure_body | &anonymous_type_declaration | &type_declaration | &variable_declaration | &procedure_declaration | &attribute_specification | &constant_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SubprogramDeclaration {
    pub identifier: Identifier,
    pub implicit_definition: Option<ImplicitDefinition>,
    #[serde(default)]
    pub interface_declarations: Vec<DeclarationNodeId>,
    pub return_type: Option<SubtypeDefinitionNodeId>,
    pub subprogram_body: Option<NodeId<SubprogramBody>>,
}

/// ```text
/// procedure_body:
///
/// sequential_statements: &[assertion_statement] | &[while_loop_statement] | &[return_statement] | &[variable_assignment_statement] | &[case_statement] | &[if_statement] | &[procedure_call_statement] | &[simple_signal_assignment_statement] | &[for_loop_statement] | &[null_statement]
/// chain: &function_declaration | &procedure_declaration | &variable_declaration | &constant_declaration | &signal_declaration
/// subprogram_specification: &procedure_declaration
/// parent: int
/// impure_depth: int
/// callees_list: &[procedure_declaration]
/// end_has_reserved_id: bool
/// declarations: &[function_declaration] | &[procedure_body] | &[attribute_declaration] | &[suspend_state_declaration] | &[type_declaration] | &[subtype_declaration] | &[use_clause] | &[variable_declaration] | &[object_alias_declaration] | &[procedure_declaration] | &[attribute_specification] | &[constant_declaration] | &[file_declaration]
/// attribute_value_chain: &attribute_value
/// suspend_flag: bool
/// ```
///
/// ```text
/// function_body:
///
/// sequential_statements: &[assertion_statement] | &[case_statement] | &[return_statement] | &[while_loop_statement] | &[variable_assignment_statement] | &[if_statement] | &[for_loop_statement] | &[null_statement]
/// chain: &component_declaration | &function_declaration | &subtype_declaration | &attribute_declaration | &anonymous_type_declaration | &type_declaration | &variable_declaration | &procedure_declaration | &attribute_specification | &constant_declaration | &signal_declaration
/// declarations: &[function_declaration] | &[subtype_declaration] | &[anonymous_type_declaration] | &[type_declaration] | &[function_body] | &[variable_declaration] | &[constant_declaration]
/// impure_depth: int
/// parent: int
/// subprogram_specification: &function_declaration
/// end_has_reserved_id: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SubprogramBody {
    pub subprogram_specification: NodeId<SubprogramDeclaration>,
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayElementResolution {}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ImplicitDefinition {
    #[serde(rename = "IIR_PREDEFINED_ACCESS_EQUALITY")]
    AccessEquality,
    #[serde(rename = "IIR_PREDEFINED_ACCESS_INEQUALITY")]
    AccessInequality,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ARRAY_CONCAT")]
    ArrayArrayConcat,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_CHAR_TO_STRING")]
    ArrayCharToString,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ELEMENT_CONCAT")]
    ArrayElementConcat,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_EQUALITY")]
    ArrayEquality,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_GREATER")]
    ArrayGreater,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_GREATER_EQUAL")]
    ArrayGreaterEqual,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_INEQUALITY")]
    ArrayInequality,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_LESS")]
    ArrayLess,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_LESS_EQUAL")]
    ArrayLessEqual,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_MAXIMUM")]
    ArrayMaximum,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_MINIMUM")]
    ArrayMinimum,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ROL")]
    ArrayRol,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ROR")]
    ArrayRor,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SLA")]
    ArraySla,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SLL")]
    ArraySll,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SRA")]
    ArraySra,
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SRL")]
    ArraySrl,
    #[serde(rename = "IIR_PREDEFINED_BIT_AND")]
    BitAnd,
    #[serde(rename = "IIR_PREDEFINED_BIT_ARRAY_MATCH_EQUALITY")]
    BitArrayMatchEquality,
    #[serde(rename = "IIR_PREDEFINED_BIT_ARRAY_MATCH_INEQUALITY")]
    BitArrayMatchInequality,
    #[serde(rename = "IIR_PREDEFINED_BIT_CONDITION")]
    BitCondition,
    #[serde(rename = "IIR_PREDEFINED_BIT_FALLING_EDGE")]
    BitFallingEdge,
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_EQUALITY")]
    BitMatchEquality,
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_GREATER")]
    BitMatchGreater,
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_GREATER_EQUAL")]
    BitMatchGreaterEqual,
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_INEQUALITY")]
    BitMatchInequality,
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_LESS")]
    BitMatchLess,
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_LESS_EQUAL")]
    BitMatchLessEqual,
    #[serde(rename = "IIR_PREDEFINED_BIT_NAND")]
    BitNand,
    #[serde(rename = "IIR_PREDEFINED_BIT_NOR")]
    BitNor,
    #[serde(rename = "IIR_PREDEFINED_BIT_NOT")]
    BitNot,
    #[serde(rename = "IIR_PREDEFINED_BIT_OR")]
    BitOr,
    #[serde(rename = "IIR_PREDEFINED_BIT_RISING_EDGE")]
    BitRisingEdge,
    #[serde(rename = "IIR_PREDEFINED_BIT_VECTOR_TO_HSTRING")]
    BitVectorToHstring,
    #[serde(rename = "IIR_PREDEFINED_BIT_VECTOR_TO_OSTRING")]
    BitVectorToOstring,
    #[serde(rename = "IIR_PREDEFINED_BIT_XNOR")]
    BitXnor,
    #[serde(rename = "IIR_PREDEFINED_BIT_XOR")]
    BitXor,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_AND")]
    BooleanAnd,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_FALLING_EDGE")]
    BooleanFallingEdge,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_NAND")]
    BooleanNand,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_NOR")]
    BooleanNor,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_NOT")]
    BooleanNot,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_OR")]
    BooleanOr,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_RISING_EDGE")]
    BooleanRisingEdge,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_XNOR")]
    BooleanXnor,
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_XOR")]
    BooleanXor,
    #[serde(rename = "IIR_PREDEFINED_DEALLOCATE")]
    Deallocate,
    #[serde(rename = "IIR_PREDEFINED_ELEMENT_ARRAY_CONCAT")]
    ElementArrayConcat,
    #[serde(rename = "IIR_PREDEFINED_ELEMENT_ELEMENT_CONCAT")]
    ElementElementConcat,
    #[serde(rename = "IIR_PREDEFINED_ENDFILE")]
    Endfile,
    #[serde(rename = "IIR_PREDEFINED_ENUM_EQUALITY")]
    EnumEquality,
    #[serde(rename = "IIR_PREDEFINED_ENUM_GREATER")]
    EnumGreater,
    #[serde(rename = "IIR_PREDEFINED_ENUM_GREATER_EQUAL")]
    EnumGreaterEqual,
    #[serde(rename = "IIR_PREDEFINED_ENUM_INEQUALITY")]
    EnumInequality,
    #[serde(rename = "IIR_PREDEFINED_ENUM_LESS")]
    EnumLess,
    #[serde(rename = "IIR_PREDEFINED_ENUM_LESS_EQUAL")]
    EnumLessEqual,
    #[serde(rename = "IIR_PREDEFINED_ENUM_MAXIMUM")]
    EnumMaximum,
    #[serde(rename = "IIR_PREDEFINED_ENUM_MINIMUM")]
    EnumMinimum,
    #[serde(rename = "IIR_PREDEFINED_ENUM_TO_STRING")]
    EnumToString,
    #[serde(rename = "IIR_PREDEFINED_ERROR")]
    Error,
    #[serde(rename = "IIR_PREDEFINED_FILE_CLOSE")]
    FileClose,
    #[serde(rename = "IIR_PREDEFINED_FILE_OPEN")]
    FileOpen,
    #[serde(rename = "IIR_PREDEFINED_FILE_OPEN_STATUS")]
    FileOpenStatus,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_ABSOLUTE")]
    FloatingAbsolute,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_DIV")]
    FloatingDiv,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_EQUALITY")]
    FloatingEquality,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_EXP")]
    FloatingExp,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_GREATER")]
    FloatingGreater,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_GREATER_EQUAL")]
    FloatingGreaterEqual,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_IDENTITY")]
    FloatingIdentity,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_INEQUALITY")]
    FloatingInequality,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_LESS")]
    FloatingLess,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_LESS_EQUAL")]
    FloatingLessEqual,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MAXIMUM")]
    FloatingMaximum,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MINIMUM")]
    FloatingMinimum,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MINUS")]
    FloatingMinus,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MUL")]
    FloatingMul,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_NEGATION")]
    FloatingNegation,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_PLUS")]
    FloatingPlus,
    #[serde(rename = "IIR_PREDEFINED_FLOATING_TO_STRING")]
    FloatingToString,
    #[serde(rename = "IIR_PREDEFINED_FLUSH")]
    Flush,
    #[serde(rename = "IIR_PREDEFINED_FOREIGN_TEXTIO_READ_REAL")]
    ForeignTextioReadReal,
    #[serde(rename = "IIR_PREDEFINED_FOREIGN_TEXTIO_WRITE_REAL")]
    ForeignTextioWriteReal,
    #[serde(rename = "IIR_PREDEFINED_FOREIGN_UNTRUNCATED_TEXT_READ")]
    ForeignUntruncatedTextRead,
    #[serde(rename = "IIR_PREDEFINED_FREQUENCY_FUNCTION")]
    FrequencyFunction,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_AND_LOG_SUV")]
    Ieee1164AndLogSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_AND_SUV")]
    Ieee1164AndSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_AND_SUV_LOG")]
    Ieee1164AndSuvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_CONDITION_OPERATOR")]
    Ieee1164ConditionOperator,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_FALLING_EDGE")]
    Ieee1164FallingEdge,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_IS_X_LOG")]
    Ieee1164IsXLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_IS_X_SLV")]
    Ieee1164IsXSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NAND_LOG_SUV")]
    Ieee1164NandLogSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NAND_SUV")]
    Ieee1164NandSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NAND_SUV_LOG")]
    Ieee1164NandSuvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NOR_LOG_SUV")]
    Ieee1164NorLogSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NOR_SUV")]
    Ieee1164NorSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NOR_SUV_LOG")]
    Ieee1164NorSuvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_OR_LOG_SUV")]
    Ieee1164OrLogSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_OR_SUV")]
    Ieee1164OrSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_OR_SUV_LOG")]
    Ieee1164OrSuvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_RISING_EDGE")]
    Ieee1164RisingEdge,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_AND")]
    Ieee1164ScalarAnd,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_NAND")]
    Ieee1164ScalarNand,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_NOR")]
    Ieee1164ScalarNor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_NOT")]
    Ieee1164ScalarNot,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_OR")]
    Ieee1164ScalarOr,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_XNOR")]
    Ieee1164ScalarXnor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_XOR")]
    Ieee1164ScalarXor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_01_LOG_LOG")]
    Ieee1164To01LogLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_01_SLV_LOG")]
    Ieee1164To01SlvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_BIT")]
    Ieee1164ToBit,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_BITVECTOR")]
    Ieee1164ToBitvector,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_HSTRING")]
    Ieee1164ToHstring,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_OSTRING")]
    Ieee1164ToOstring,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDLOGICVECTOR_BV")]
    Ieee1164ToStdlogicvectorBv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDLOGICVECTOR_SUV")]
    Ieee1164ToStdlogicvectorSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDULOGIC")]
    Ieee1164ToStdulogic,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDULOGICVECTOR_BV")]
    Ieee1164ToStdulogicvectorBv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDULOGICVECTOR_SLV")]
    Ieee1164ToStdulogicvectorSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_BIT_LOG")]
    Ieee1164ToUx01BitLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_BV_SLV")]
    Ieee1164ToUx01BvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_BV_SUV")]
    Ieee1164ToUx01BvSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_LOG")]
    Ieee1164ToUx01Log,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_SLV")]
    Ieee1164ToUx01Slv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_SUV")]
    Ieee1164ToUx01Suv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_BIT_LOG")]
    Ieee1164ToX01BitLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_BV_SLV")]
    Ieee1164ToX01BvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_BV_SUV")]
    Ieee1164ToX01BvSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_LOG")]
    Ieee1164ToX01Log,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_SLV")]
    Ieee1164ToX01Slv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_SUV")]
    Ieee1164ToX01Suv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_BIT_LOG")]
    Ieee1164ToX01zBitLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_BV_SLV")]
    Ieee1164ToX01zBvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_BV_SUV")]
    Ieee1164ToX01zBvSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_LOG")]
    Ieee1164ToX01zLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_SLV")]
    Ieee1164ToX01zSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_SUV")]
    Ieee1164ToX01zSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_AND")]
    Ieee1164VectorAnd,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_NAND")]
    Ieee1164VectorNand,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_NOR")]
    Ieee1164VectorNor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_NOT")]
    Ieee1164VectorNot,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_OR")]
    Ieee1164VectorOr,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_ROL")]
    Ieee1164VectorRol,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_ROR")]
    Ieee1164VectorRor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_SLL")]
    Ieee1164VectorSll,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_SRL")]
    Ieee1164VectorSrl,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_XNOR")]
    Ieee1164VectorXnor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_XOR")]
    Ieee1164VectorXor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XNOR_LOG_SUV")]
    Ieee1164XnorLogSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XNOR_SUV")]
    Ieee1164XnorSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XNOR_SUV_LOG")]
    Ieee1164XnorSuvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XOR_LOG_SUV")]
    Ieee1164XorLogSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XOR_SUV")]
    Ieee1164XorSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XOR_SUV_LOG")]
    Ieee1164XorSuvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCCOS")]
    IeeeMathRealArccos,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCCOSH")]
    IeeeMathRealArccosh,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCSIN")]
    IeeeMathRealArcsin,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCSINH")]
    IeeeMathRealArcsinh,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCTAN")]
    IeeeMathRealArctan,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCTAN_REAL_REAL")]
    IeeeMathRealArctanRealReal,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCTANH")]
    IeeeMathRealArctanh,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_CBRT")]
    IeeeMathRealCbrt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_CEIL")]
    IeeeMathRealCeil,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_COS")]
    IeeeMathRealCos,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_COSH")]
    IeeeMathRealCosh,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_EXP")]
    IeeeMathRealExp,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_FLOOR")]
    IeeeMathRealFloor,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG")]
    IeeeMathRealLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG_REAL_REAL")]
    IeeeMathRealLogRealReal,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG10")]
    IeeeMathRealLog10,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG2")]
    IeeeMathRealLog2,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_MOD")]
    IeeeMathRealMod,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_POW_INT_REAL")]
    IeeeMathRealPowIntReal,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_POW_REAL_REAL")]
    IeeeMathRealPowRealReal,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_REALMAX")]
    IeeeMathRealRealmax,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_REALMIN")]
    IeeeMathRealRealmin,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ROUND")]
    IeeeMathRealRound,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SIGN")]
    IeeeMathRealSign,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SIN")]
    IeeeMathRealSin,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SINH")]
    IeeeMathRealSinh,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SQRT")]
    IeeeMathRealSqrt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_TAN")]
    IeeeMathRealTan,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_TANH")]
    IeeeMathRealTanh,
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_TRUNC")]
    IeeeMathRealTrunc,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOINT_SGN_INT")]
    IeeeNumericBitToIntSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOINT_UNS_NAT")]
    IeeeNumericBitToIntUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOSGN_INT_NAT_SGN")]
    IeeeNumericBitToSgnIntNatSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOSGN_INT_SGN_SGN")]
    IeeeNumericBitToSgnIntSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOUNS_NAT_NAT_UNS")]
    IeeeNumericBitToUnsNatNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOUNS_NAT_UNS_UNS")]
    IeeeNumericBitToUnsNatUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ABS_SGN")]
    IeeeNumericStdAbsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_INT_SGN")]
    IeeeNumericStdAddIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_LOG_SGN")]
    IeeeNumericStdAddLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_LOG_UNS")]
    IeeeNumericStdAddLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_NAT_UNS")]
    IeeeNumericStdAddNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_SGN_INT")]
    IeeeNumericStdAddSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_SGN_LOG")]
    IeeeNumericStdAddSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_SGN_SGN")]
    IeeeNumericStdAddSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_UNS_LOG")]
    IeeeNumericStdAddUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_UNS_NAT")]
    IeeeNumericStdAddUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_UNS_UNS")]
    IeeeNumericStdAddUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_LOG_SGN")]
    IeeeNumericStdAndLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_LOG_UNS")]
    IeeeNumericStdAndLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_SGN")]
    IeeeNumericStdAndSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_SGN_LOG")]
    IeeeNumericStdAndSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_SGN_SGN")]
    IeeeNumericStdAndSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_UNS")]
    IeeeNumericStdAndUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_UNS_LOG")]
    IeeeNumericStdAndUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_UNS_UNS")]
    IeeeNumericStdAndUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_INT_SGN")]
    IeeeNumericStdDivIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_NAT_UNS")]
    IeeeNumericStdDivNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_SGN_INT")]
    IeeeNumericStdDivSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_SGN_SGN")]
    IeeeNumericStdDivSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_UNS_NAT")]
    IeeeNumericStdDivUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_UNS_UNS")]
    IeeeNumericStdDivUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_INT_SGN")]
    IeeeNumericStdEqIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_NAT_UNS")]
    IeeeNumericStdEqNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_SGN_INT")]
    IeeeNumericStdEqSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_SGN_SGN")]
    IeeeNumericStdEqSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_UNS_NAT")]
    IeeeNumericStdEqUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_UNS_UNS")]
    IeeeNumericStdEqUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_LEFTMOST_SGN")]
    IeeeNumericStdFindLeftmostSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_LEFTMOST_UNS")]
    IeeeNumericStdFindLeftmostUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_RIGHTMOST_SGN")]
    IeeeNumericStdFindRightmostSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_RIGHTMOST_UNS")]
    IeeeNumericStdFindRightmostUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_INT_SGN")]
    IeeeNumericStdGeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_NAT_UNS")]
    IeeeNumericStdGeNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_SGN_INT")]
    IeeeNumericStdGeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_SGN_SGN")]
    IeeeNumericStdGeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_UNS_NAT")]
    IeeeNumericStdGeUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_UNS_UNS")]
    IeeeNumericStdGeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_INT_SGN")]
    IeeeNumericStdGtIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_NAT_UNS")]
    IeeeNumericStdGtNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_SGN_INT")]
    IeeeNumericStdGtSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_SGN_SGN")]
    IeeeNumericStdGtSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_UNS_NAT")]
    IeeeNumericStdGtUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_UNS_UNS")]
    IeeeNumericStdGtUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_IS_X_SGN")]
    IeeeNumericStdIsXSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_IS_X_UNS")]
    IeeeNumericStdIsXUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_INT_SGN")]
    IeeeNumericStdLeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_NAT_UNS")]
    IeeeNumericStdLeNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_SGN_INT")]
    IeeeNumericStdLeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_SGN_SGN")]
    IeeeNumericStdLeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_UNS_NAT")]
    IeeeNumericStdLeUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_UNS_UNS")]
    IeeeNumericStdLeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_INT_SGN")]
    IeeeNumericStdLtIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_NAT_UNS")]
    IeeeNumericStdLtNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_SGN_INT")]
    IeeeNumericStdLtSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_SGN_SGN")]
    IeeeNumericStdLtSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_UNS_NAT")]
    IeeeNumericStdLtUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_UNS_UNS")]
    IeeeNumericStdLtUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_INT_SGN")]
    IeeeNumericStdMatchEqIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_NAT_UNS")]
    IeeeNumericStdMatchEqNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_SGN_INT")]
    IeeeNumericStdMatchEqSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_SGN_SGN")]
    IeeeNumericStdMatchEqSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_UNS_NAT")]
    IeeeNumericStdMatchEqUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_UNS_UNS")]
    IeeeNumericStdMatchEqUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_INT_SGN")]
    IeeeNumericStdMatchGeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_NAT_UNS")]
    IeeeNumericStdMatchGeNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_SGN_INT")]
    IeeeNumericStdMatchGeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_SGN_SGN")]
    IeeeNumericStdMatchGeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_UNS_NAT")]
    IeeeNumericStdMatchGeUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_UNS_UNS")]
    IeeeNumericStdMatchGeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_INT_SGN")]
    IeeeNumericStdMatchGtIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_NAT_UNS")]
    IeeeNumericStdMatchGtNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_SGN_INT")]
    IeeeNumericStdMatchGtSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_SGN_SGN")]
    IeeeNumericStdMatchGtSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_UNS_NAT")]
    IeeeNumericStdMatchGtUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_UNS_UNS")]
    IeeeNumericStdMatchGtUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_INT_SGN")]
    IeeeNumericStdMatchLeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_NAT_UNS")]
    IeeeNumericStdMatchLeNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_SGN_INT")]
    IeeeNumericStdMatchLeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_SGN_SGN")]
    IeeeNumericStdMatchLeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_UNS_NAT")]
    IeeeNumericStdMatchLeUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_UNS_UNS")]
    IeeeNumericStdMatchLeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LOG")]
    IeeeNumericStdMatchLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_INT_SGN")]
    IeeeNumericStdMatchLtIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_NAT_UNS")]
    IeeeNumericStdMatchLtNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_SGN_INT")]
    IeeeNumericStdMatchLtSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_SGN_SGN")]
    IeeeNumericStdMatchLtSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_UNS_NAT")]
    IeeeNumericStdMatchLtUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_UNS_UNS")]
    IeeeNumericStdMatchLtUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_INT_SGN")]
    IeeeNumericStdMatchNeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_NAT_UNS")]
    IeeeNumericStdMatchNeNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_SGN_INT")]
    IeeeNumericStdMatchNeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_SGN_SGN")]
    IeeeNumericStdMatchNeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_UNS_NAT")]
    IeeeNumericStdMatchNeUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_UNS_UNS")]
    IeeeNumericStdMatchNeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_SGN")]
    IeeeNumericStdMatchSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_SLV")]
    IeeeNumericStdMatchSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_SUV")]
    IeeeNumericStdMatchSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_UNS")]
    IeeeNumericStdMatchUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_INT_SGN")]
    IeeeNumericStdMaxIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_NAT_UNS")]
    IeeeNumericStdMaxNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_SGN_INT")]
    IeeeNumericStdMaxSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_SGN_SGN")]
    IeeeNumericStdMaxSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_UNS_NAT")]
    IeeeNumericStdMaxUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_UNS_UNS")]
    IeeeNumericStdMaxUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_INT_SGN")]
    IeeeNumericStdMinIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_NAT_UNS")]
    IeeeNumericStdMinNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_SGN_INT")]
    IeeeNumericStdMinSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_SGN_SGN")]
    IeeeNumericStdMinSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_UNS_NAT")]
    IeeeNumericStdMinUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_UNS_UNS")]
    IeeeNumericStdMinUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_INT_SGN")]
    IeeeNumericStdModIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_NAT_UNS")]
    IeeeNumericStdModNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_SGN_INT")]
    IeeeNumericStdModSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_SGN_SGN")]
    IeeeNumericStdModSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_UNS_NAT")]
    IeeeNumericStdModUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_UNS_UNS")]
    IeeeNumericStdModUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_INT_SGN")]
    IeeeNumericStdMulIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_NAT_UNS")]
    IeeeNumericStdMulNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_SGN_INT")]
    IeeeNumericStdMulSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_SGN_SGN")]
    IeeeNumericStdMulSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_UNS_NAT")]
    IeeeNumericStdMulUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_UNS_UNS")]
    IeeeNumericStdMulUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_LOG_SGN")]
    IeeeNumericStdNandLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_LOG_UNS")]
    IeeeNumericStdNandLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_SGN")]
    IeeeNumericStdNandSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_SGN_LOG")]
    IeeeNumericStdNandSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_SGN_SGN")]
    IeeeNumericStdNandSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_UNS")]
    IeeeNumericStdNandUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_UNS_LOG")]
    IeeeNumericStdNandUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_UNS_UNS")]
    IeeeNumericStdNandUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_INT_SGN")]
    IeeeNumericStdNeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_NAT_UNS")]
    IeeeNumericStdNeNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_SGN_INT")]
    IeeeNumericStdNeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_SGN_SGN")]
    IeeeNumericStdNeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_UNS_NAT")]
    IeeeNumericStdNeUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_UNS_UNS")]
    IeeeNumericStdNeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NEG_SGN")]
    IeeeNumericStdNegSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NEG_UNS")]
    IeeeNumericStdNegUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_LOG_SGN")]
    IeeeNumericStdNorLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_LOG_UNS")]
    IeeeNumericStdNorLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_SGN")]
    IeeeNumericStdNorSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_SGN_LOG")]
    IeeeNumericStdNorSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_SGN_SGN")]
    IeeeNumericStdNorSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_UNS")]
    IeeeNumericStdNorUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_UNS_LOG")]
    IeeeNumericStdNorUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_UNS_UNS")]
    IeeeNumericStdNorUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOT_SGN")]
    IeeeNumericStdNotSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOT_UNS")]
    IeeeNumericStdNotUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_LOG_SGN")]
    IeeeNumericStdOrLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_LOG_UNS")]
    IeeeNumericStdOrLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_SGN")]
    IeeeNumericStdOrSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_SGN_LOG")]
    IeeeNumericStdOrSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_SGN_SGN")]
    IeeeNumericStdOrSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_UNS")]
    IeeeNumericStdOrUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_UNS_LOG")]
    IeeeNumericStdOrUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_UNS_UNS")]
    IeeeNumericStdOrUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_INT_SGN")]
    IeeeNumericStdRemIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_NAT_UNS")]
    IeeeNumericStdRemNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_SGN_INT")]
    IeeeNumericStdRemSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_SGN_SGN")]
    IeeeNumericStdRemSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_UNS_NAT")]
    IeeeNumericStdRemUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_UNS_UNS")]
    IeeeNumericStdRemUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_SGN_NAT")]
    IeeeNumericStdResizeSgnNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_SGN_SGN")]
    IeeeNumericStdResizeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_UNS_NAT")]
    IeeeNumericStdResizeUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_UNS_UNS")]
    IeeeNumericStdResizeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROL_SGN_INT")]
    IeeeNumericStdRolSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROL_UNS_INT")]
    IeeeNumericStdRolUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROR_SGN_INT")]
    IeeeNumericStdRorSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROR_UNS_INT")]
    IeeeNumericStdRorUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_LEFT_SGN_NAT")]
    IeeeNumericStdRotLeftSgnNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_LEFT_UNS_NAT")]
    IeeeNumericStdRotLeftUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_RIGHT_SGN_NAT")]
    IeeeNumericStdRotRightSgnNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_RIGHT_UNS_NAT")]
    IeeeNumericStdRotRightUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_LEFT_SGN_NAT")]
    IeeeNumericStdShfLeftSgnNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_LEFT_UNS_NAT")]
    IeeeNumericStdShfLeftUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_RIGHT_SGN_NAT")]
    IeeeNumericStdShfRightSgnNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_RIGHT_UNS_NAT")]
    IeeeNumericStdShfRightUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLA_SGN_INT")]
    IeeeNumericStdSlaSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLA_UNS_INT")]
    IeeeNumericStdSlaUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLL_SGN_INT")]
    IeeeNumericStdSllSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLL_UNS_INT")]
    IeeeNumericStdSllUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRA_SGN_INT")]
    IeeeNumericStdSraSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRA_UNS_INT")]
    IeeeNumericStdSraUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRL_SGN_INT")]
    IeeeNumericStdSrlSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRL_UNS_INT")]
    IeeeNumericStdSrlUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_INT_SGN")]
    IeeeNumericStdSubIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_LOG_SGN")]
    IeeeNumericStdSubLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_LOG_UNS")]
    IeeeNumericStdSubLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_NAT_UNS")]
    IeeeNumericStdSubNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_SGN_INT")]
    IeeeNumericStdSubSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_SGN_LOG")]
    IeeeNumericStdSubSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_SGN_SGN")]
    IeeeNumericStdSubSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_UNS_LOG")]
    IeeeNumericStdSubUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_UNS_NAT")]
    IeeeNumericStdSubUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_UNS_UNS")]
    IeeeNumericStdSubUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_01_SGN")]
    IeeeNumericStdTo01Sgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_01_UNS")]
    IeeeNumericStdTo01Uns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_HSTRING_SGN")]
    IeeeNumericStdToHstringSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_HSTRING_UNS")]
    IeeeNumericStdToHstringUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_OSTRING_SGN")]
    IeeeNumericStdToOstringSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_OSTRING_UNS")]
    IeeeNumericStdToOstringUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_UX01_SGN")]
    IeeeNumericStdToUx01Sgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_UX01_UNS")]
    IeeeNumericStdToUx01Uns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01_SGN")]
    IeeeNumericStdToX01Sgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01_UNS")]
    IeeeNumericStdToX01Uns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01Z_SGN")]
    IeeeNumericStdToX01zSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01Z_UNS")]
    IeeeNumericStdToX01zUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOINT_SGN_INT")]
    IeeeNumericStdToIntSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOINT_UNS_NAT")]
    IeeeNumericStdToIntUnsNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOSGN_INT_NAT_SGN")]
    IeeeNumericStdToSgnIntNatSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOSGN_INT_SGN_SGN")]
    IeeeNumericStdToSgnIntSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOUNS_NAT_NAT_UNS")]
    IeeeNumericStdToUnsNatNatUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOUNS_NAT_UNS_UNS")]
    IeeeNumericStdToUnsNatUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ADD_NAT_SLV")]
    IeeeNumericStdUnsignedAddNatSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ADD_SLV_NAT")]
    IeeeNumericStdUnsignedAddSlvNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ADD_SLV_SLV")]
    IeeeNumericStdUnsignedAddSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_FIND_LEFTMOST")]
    IeeeNumericStdUnsignedFindLeftmost,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_FIND_RIGHTMOST")]
    IeeeNumericStdUnsignedFindRightmost,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_MAXIMUM_SLV_SLV")]
    IeeeNumericStdUnsignedMaximumSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_MINIMUM_SLV_SLV")]
    IeeeNumericStdUnsignedMinimumSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_RESIZE_SLV_NAT")]
    IeeeNumericStdUnsignedResizeSlvNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_RESIZE_SLV_SLV")]
    IeeeNumericStdUnsignedResizeSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ROTATE_LEFT")]
    IeeeNumericStdUnsignedRotateLeft,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ROTATE_RIGHT")]
    IeeeNumericStdUnsignedRotateRight,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SHIFT_LEFT")]
    IeeeNumericStdUnsignedShiftLeft,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SHIFT_RIGHT")]
    IeeeNumericStdUnsignedShiftRight,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SUB_NAT_SLV")]
    IeeeNumericStdUnsignedSubNatSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SUB_SLV_NAT")]
    IeeeNumericStdUnsignedSubSlvNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SUB_SLV_SLV")]
    IeeeNumericStdUnsignedSubSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_INTEGER_SLV_NAT")]
    IeeeNumericStdUnsignedToIntegerSlvNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SLV_NAT_NAT")]
    IeeeNumericStdUnsignedToSlvNatNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SLV_NAT_SLV")]
    IeeeNumericStdUnsignedToSlvNatSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SUV_NAT_NAT")]
    IeeeNumericStdUnsignedToSuvNatNat,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SUV_NAT_SUV")]
    IeeeNumericStdUnsignedToSuvNatSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_LOG_SGN")]
    IeeeNumericStdXnorLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_LOG_UNS")]
    IeeeNumericStdXnorLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_SGN")]
    IeeeNumericStdXnorSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_SGN_LOG")]
    IeeeNumericStdXnorSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_SGN_SGN")]
    IeeeNumericStdXnorSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_UNS")]
    IeeeNumericStdXnorUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_UNS_LOG")]
    IeeeNumericStdXnorUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_UNS_UNS")]
    IeeeNumericStdXnorUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_LOG_SGN")]
    IeeeNumericStdXorLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_LOG_UNS")]
    IeeeNumericStdXorLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_SGN")]
    IeeeNumericStdXorSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_SGN_LOG")]
    IeeeNumericStdXorSgnLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_SGN_SGN")]
    IeeeNumericStdXorSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_UNS")]
    IeeeNumericStdXorUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_UNS_LOG")]
    IeeeNumericStdXorUnsLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_UNS_UNS")]
    IeeeNumericStdXorUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ABS_SGN_SGN")]
    IeeeStdLogicArithAbsSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ABS_SGN_SLV")]
    IeeeStdLogicArithAbsSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_SGN_SGN")]
    IeeeStdLogicArithAddIntSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_SGN_SLV")]
    IeeeStdLogicArithAddIntSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_UNS_SLV")]
    IeeeStdLogicArithAddIntUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_UNS_UNS")]
    IeeeStdLogicArithAddIntUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_SGN_SGN")]
    IeeeStdLogicArithAddLogSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_SGN_SLV")]
    IeeeStdLogicArithAddLogSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_UNS_SLV")]
    IeeeStdLogicArithAddLogUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_UNS_UNS")]
    IeeeStdLogicArithAddLogUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_INT_SGN")]
    IeeeStdLogicArithAddSgnIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_INT_SLV")]
    IeeeStdLogicArithAddSgnIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_LOG_SGN")]
    IeeeStdLogicArithAddSgnLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_LOG_SLV")]
    IeeeStdLogicArithAddSgnLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_SGN_SGN")]
    IeeeStdLogicArithAddSgnSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_SGN_SLV")]
    IeeeStdLogicArithAddSgnSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_UNS_SGN")]
    IeeeStdLogicArithAddSgnUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_UNS_SLV")]
    IeeeStdLogicArithAddSgnUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_INT_SLV")]
    IeeeStdLogicArithAddUnsIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_INT_UNS")]
    IeeeStdLogicArithAddUnsIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_LOG_SLV")]
    IeeeStdLogicArithAddUnsLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_LOG_UNS")]
    IeeeStdLogicArithAddUnsLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_SGN_SGN")]
    IeeeStdLogicArithAddUnsSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_SGN_SLV")]
    IeeeStdLogicArithAddUnsSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_UNS_SLV")]
    IeeeStdLogicArithAddUnsUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_UNS_UNS")]
    IeeeStdLogicArithAddUnsUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_INT")]
    IeeeStdLogicArithConvIntegerInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_LOG")]
    IeeeStdLogicArithConvIntegerLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_SGN")]
    IeeeStdLogicArithConvIntegerSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_UNS")]
    IeeeStdLogicArithConvIntegerUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_INT")]
    IeeeStdLogicArithConvSignedInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_LOG")]
    IeeeStdLogicArithConvSignedLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_SGN")]
    IeeeStdLogicArithConvSignedSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_UNS")]
    IeeeStdLogicArithConvSignedUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_INT")]
    IeeeStdLogicArithConvUnsignedInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_LOG")]
    IeeeStdLogicArithConvUnsignedLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_SGN")]
    IeeeStdLogicArithConvUnsignedSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_UNS")]
    IeeeStdLogicArithConvUnsignedUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_INT")]
    IeeeStdLogicArithConvVectorInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_LOG")]
    IeeeStdLogicArithConvVectorLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_SGN")]
    IeeeStdLogicArithConvVectorSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_UNS")]
    IeeeStdLogicArithConvVectorUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_INT_SGN")]
    IeeeStdLogicArithEqIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_INT_UNS")]
    IeeeStdLogicArithEqIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_SGN_INT")]
    IeeeStdLogicArithEqSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_SGN_SGN")]
    IeeeStdLogicArithEqSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_SGN_UNS")]
    IeeeStdLogicArithEqSgnUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_UNS_INT")]
    IeeeStdLogicArithEqUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_UNS_SGN")]
    IeeeStdLogicArithEqUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_UNS_UNS")]
    IeeeStdLogicArithEqUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EXT")]
    IeeeStdLogicArithExt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_INT_SGN")]
    IeeeStdLogicArithGeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_INT_UNS")]
    IeeeStdLogicArithGeIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_SGN_INT")]
    IeeeStdLogicArithGeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_SGN_SGN")]
    IeeeStdLogicArithGeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_SGN_UNS")]
    IeeeStdLogicArithGeSgnUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_UNS_INT")]
    IeeeStdLogicArithGeUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_UNS_SGN")]
    IeeeStdLogicArithGeUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_UNS_UNS")]
    IeeeStdLogicArithGeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_INT_SGN")]
    IeeeStdLogicArithGtIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_INT_UNS")]
    IeeeStdLogicArithGtIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_SGN_INT")]
    IeeeStdLogicArithGtSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_SGN_SGN")]
    IeeeStdLogicArithGtSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_SGN_UNS")]
    IeeeStdLogicArithGtSgnUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_UNS_INT")]
    IeeeStdLogicArithGtUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_UNS_SGN")]
    IeeeStdLogicArithGtUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_UNS_UNS")]
    IeeeStdLogicArithGtUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_SGN_SGN")]
    IeeeStdLogicArithIdSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_SGN_SLV")]
    IeeeStdLogicArithIdSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_UNS_SLV")]
    IeeeStdLogicArithIdUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_UNS_UNS")]
    IeeeStdLogicArithIdUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_INT_SGN")]
    IeeeStdLogicArithLeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_INT_UNS")]
    IeeeStdLogicArithLeIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_SGN_INT")]
    IeeeStdLogicArithLeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_SGN_SGN")]
    IeeeStdLogicArithLeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_SGN_UNS")]
    IeeeStdLogicArithLeSgnUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_UNS_INT")]
    IeeeStdLogicArithLeUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_UNS_SGN")]
    IeeeStdLogicArithLeUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_UNS_UNS")]
    IeeeStdLogicArithLeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_INT_SGN")]
    IeeeStdLogicArithLtIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_INT_UNS")]
    IeeeStdLogicArithLtIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_SGN_INT")]
    IeeeStdLogicArithLtSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_SGN_SGN")]
    IeeeStdLogicArithLtSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_SGN_UNS")]
    IeeeStdLogicArithLtSgnUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_UNS_INT")]
    IeeeStdLogicArithLtUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_UNS_SGN")]
    IeeeStdLogicArithLtUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_UNS_UNS")]
    IeeeStdLogicArithLtUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_SGN_SGN")]
    IeeeStdLogicArithMulSgnSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_SGN_SLV")]
    IeeeStdLogicArithMulSgnSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_UNS_SGN")]
    IeeeStdLogicArithMulSgnUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_UNS_SLV")]
    IeeeStdLogicArithMulSgnUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_SGN_SGN")]
    IeeeStdLogicArithMulUnsSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_SGN_SLV")]
    IeeeStdLogicArithMulUnsSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_UNS_SLV")]
    IeeeStdLogicArithMulUnsUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_UNS_UNS")]
    IeeeStdLogicArithMulUnsUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_INT_SGN")]
    IeeeStdLogicArithNeIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_INT_UNS")]
    IeeeStdLogicArithNeIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_SGN_INT")]
    IeeeStdLogicArithNeSgnInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_SGN_SGN")]
    IeeeStdLogicArithNeSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_SGN_UNS")]
    IeeeStdLogicArithNeSgnUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_UNS_INT")]
    IeeeStdLogicArithNeUnsInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_UNS_SGN")]
    IeeeStdLogicArithNeUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_UNS_UNS")]
    IeeeStdLogicArithNeUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NEG_SGN_SGN")]
    IeeeStdLogicArithNegSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NEG_SGN_SLV")]
    IeeeStdLogicArithNegSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHL_SGN")]
    IeeeStdLogicArithShlSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHL_UNS")]
    IeeeStdLogicArithShlUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHR_SGN")]
    IeeeStdLogicArithShrSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHR_UNS")]
    IeeeStdLogicArithShrUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_SGN_SGN")]
    IeeeStdLogicArithSubIntSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_SGN_SLV")]
    IeeeStdLogicArithSubIntSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_UNS_SLV")]
    IeeeStdLogicArithSubIntUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_UNS_UNS")]
    IeeeStdLogicArithSubIntUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_SGN_SGN")]
    IeeeStdLogicArithSubLogSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_SGN_SLV")]
    IeeeStdLogicArithSubLogSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_UNS_SLV")]
    IeeeStdLogicArithSubLogUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_UNS_UNS")]
    IeeeStdLogicArithSubLogUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_INT_SGN")]
    IeeeStdLogicArithSubSgnIntSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_INT_SLV")]
    IeeeStdLogicArithSubSgnIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_LOG_SGN")]
    IeeeStdLogicArithSubSgnLogSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_LOG_SLV")]
    IeeeStdLogicArithSubSgnLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_SGN_SGN")]
    IeeeStdLogicArithSubSgnSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_SGN_SLV")]
    IeeeStdLogicArithSubSgnSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_UNS_SGN")]
    IeeeStdLogicArithSubSgnUnsSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_UNS_SLV")]
    IeeeStdLogicArithSubSgnUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_INT_SLV")]
    IeeeStdLogicArithSubUnsIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_INT_UNS")]
    IeeeStdLogicArithSubUnsIntUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_LOG_SLV")]
    IeeeStdLogicArithSubUnsLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_LOG_UNS")]
    IeeeStdLogicArithSubUnsLogUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_SGN_SGN")]
    IeeeStdLogicArithSubUnsSgnSgn,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_SGN_SLV")]
    IeeeStdLogicArithSubUnsSgnSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_UNS_SLV")]
    IeeeStdLogicArithSubUnsUnsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_UNS_UNS")]
    IeeeStdLogicArithSubUnsUnsUns,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SXT")]
    IeeeStdLogicArithSxt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_AND_REDUCE_SLV")]
    IeeeStdLogicMiscAndReduceSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_AND_REDUCE_SUV")]
    IeeeStdLogicMiscAndReduceSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NAND_REDUCE_SLV")]
    IeeeStdLogicMiscNandReduceSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NAND_REDUCE_SUV")]
    IeeeStdLogicMiscNandReduceSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NOR_REDUCE_SLV")]
    IeeeStdLogicMiscNorReduceSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NOR_REDUCE_SUV")]
    IeeeStdLogicMiscNorReduceSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_OR_REDUCE_SLV")]
    IeeeStdLogicMiscOrReduceSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_OR_REDUCE_SUV")]
    IeeeStdLogicMiscOrReduceSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XNOR_REDUCE_SLV")]
    IeeeStdLogicMiscXnorReduceSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XNOR_REDUCE_SUV")]
    IeeeStdLogicMiscXnorReduceSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XOR_REDUCE_SLV")]
    IeeeStdLogicMiscXorReduceSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XOR_REDUCE_SUV")]
    IeeeStdLogicMiscXorReduceSuv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ABS_SLV")]
    IeeeStdLogicSignedAbsSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_INT_SLV")]
    IeeeStdLogicSignedAddIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_LOG_SLV")]
    IeeeStdLogicSignedAddLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_SLV_INT")]
    IeeeStdLogicSignedAddSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_SLV_LOG")]
    IeeeStdLogicSignedAddSlvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_SLV_SLV")]
    IeeeStdLogicSignedAddSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_CONV_INTEGER")]
    IeeeStdLogicSignedConvInteger,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_EQ_INT_SLV")]
    IeeeStdLogicSignedEqIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_EQ_SLV_INT")]
    IeeeStdLogicSignedEqSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_EQ_SLV_SLV")]
    IeeeStdLogicSignedEqSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GE_INT_SLV")]
    IeeeStdLogicSignedGeIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GE_SLV_INT")]
    IeeeStdLogicSignedGeSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GE_SLV_SLV")]
    IeeeStdLogicSignedGeSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GT_INT_SLV")]
    IeeeStdLogicSignedGtIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GT_SLV_INT")]
    IeeeStdLogicSignedGtSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GT_SLV_SLV")]
    IeeeStdLogicSignedGtSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ID_SLV")]
    IeeeStdLogicSignedIdSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LE_INT_SLV")]
    IeeeStdLogicSignedLeIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LE_SLV_INT")]
    IeeeStdLogicSignedLeSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LE_SLV_SLV")]
    IeeeStdLogicSignedLeSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LT_INT_SLV")]
    IeeeStdLogicSignedLtIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LT_SLV_INT")]
    IeeeStdLogicSignedLtSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LT_SLV_SLV")]
    IeeeStdLogicSignedLtSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_MUL_SLV_SLV")]
    IeeeStdLogicSignedMulSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NE_INT_SLV")]
    IeeeStdLogicSignedNeIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NE_SLV_INT")]
    IeeeStdLogicSignedNeSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NE_SLV_SLV")]
    IeeeStdLogicSignedNeSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NEG_SLV")]
    IeeeStdLogicSignedNegSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SHL")]
    IeeeStdLogicSignedShl,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SHR")]
    IeeeStdLogicSignedShr,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_INT_SLV")]
    IeeeStdLogicSignedSubIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_LOG_SLV")]
    IeeeStdLogicSignedSubLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_SLV_INT")]
    IeeeStdLogicSignedSubSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_SLV_LOG")]
    IeeeStdLogicSignedSubSlvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_SLV_SLV")]
    IeeeStdLogicSignedSubSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_INT_SLV")]
    IeeeStdLogicUnsignedAddIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_LOG_SLV")]
    IeeeStdLogicUnsignedAddLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_SLV_INT")]
    IeeeStdLogicUnsignedAddSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_SLV_LOG")]
    IeeeStdLogicUnsignedAddSlvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_SLV_SLV")]
    IeeeStdLogicUnsignedAddSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_CONV_INTEGER")]
    IeeeStdLogicUnsignedConvInteger,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_EQ_INT_SLV")]
    IeeeStdLogicUnsignedEqIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_EQ_SLV_INT")]
    IeeeStdLogicUnsignedEqSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_EQ_SLV_SLV")]
    IeeeStdLogicUnsignedEqSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GE_INT_SLV")]
    IeeeStdLogicUnsignedGeIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GE_SLV_INT")]
    IeeeStdLogicUnsignedGeSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GE_SLV_SLV")]
    IeeeStdLogicUnsignedGeSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GT_INT_SLV")]
    IeeeStdLogicUnsignedGtIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GT_SLV_INT")]
    IeeeStdLogicUnsignedGtSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GT_SLV_SLV")]
    IeeeStdLogicUnsignedGtSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ID_SLV")]
    IeeeStdLogicUnsignedIdSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LE_INT_SLV")]
    IeeeStdLogicUnsignedLeIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LE_SLV_INT")]
    IeeeStdLogicUnsignedLeSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LE_SLV_SLV")]
    IeeeStdLogicUnsignedLeSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LT_INT_SLV")]
    IeeeStdLogicUnsignedLtIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LT_SLV_INT")]
    IeeeStdLogicUnsignedLtSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LT_SLV_SLV")]
    IeeeStdLogicUnsignedLtSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_MUL_SLV_SLV")]
    IeeeStdLogicUnsignedMulSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_NE_INT_SLV")]
    IeeeStdLogicUnsignedNeIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_NE_SLV_INT")]
    IeeeStdLogicUnsignedNeSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_NE_SLV_SLV")]
    IeeeStdLogicUnsignedNeSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SHL")]
    IeeeStdLogicUnsignedShl,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SHR")]
    IeeeStdLogicUnsignedShr,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_INT_SLV")]
    IeeeStdLogicUnsignedSubIntSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_LOG_SLV")]
    IeeeStdLogicUnsignedSubLogSlv,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_SLV_INT")]
    IeeeStdLogicUnsignedSubSlvInt,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_SLV_LOG")]
    IeeeStdLogicUnsignedSubSlvLog,
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_SLV_SLV")]
    IeeeStdLogicUnsignedSubSlvSlv,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_ABSOLUTE")]
    IntegerAbsolute,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_DIV")]
    IntegerDiv,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_EQUALITY")]
    IntegerEquality,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_EXP")]
    IntegerExp,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_GREATER")]
    IntegerGreater,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_GREATER_EQUAL")]
    IntegerGreaterEqual,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_IDENTITY")]
    IntegerIdentity,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_INEQUALITY")]
    IntegerInequality,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_LESS")]
    IntegerLess,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_LESS_EQUAL")]
    IntegerLessEqual,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MAXIMUM")]
    IntegerMaximum,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MINIMUM")]
    IntegerMinimum,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MINUS")]
    IntegerMinus,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MOD")]
    IntegerMod,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MUL")]
    IntegerMul,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_NEGATION")]
    IntegerNegation,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_PHYSICAL_MUL")]
    IntegerPhysicalMul,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_PLUS")]
    IntegerPlus,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_REM")]
    IntegerRem,
    #[serde(rename = "IIR_PREDEFINED_INTEGER_TO_STRING")]
    IntegerToString,
    #[serde(rename = "IIR_PREDEFINED_NOW_FUNCTION")]
    NowFunction,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_ABSOLUTE")]
    PhysicalAbsolute,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_EQUALITY")]
    PhysicalEquality,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_GREATER")]
    PhysicalGreater,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_GREATER_EQUAL")]
    PhysicalGreaterEqual,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_IDENTITY")]
    PhysicalIdentity,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_INEQUALITY")]
    PhysicalInequality,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_INTEGER_DIV")]
    PhysicalIntegerDiv,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_INTEGER_MUL")]
    PhysicalIntegerMul,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_LESS")]
    PhysicalLess,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_LESS_EQUAL")]
    PhysicalLessEqual,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MAXIMUM")]
    PhysicalMaximum,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MINIMUM")]
    PhysicalMinimum,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MINUS")]
    PhysicalMinus,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MOD")]
    PhysicalMod,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_NEGATION")]
    PhysicalNegation,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_PHYSICAL_DIV")]
    PhysicalPhysicalDiv,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_PLUS")]
    PhysicalPlus,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_REAL_DIV")]
    PhysicalRealDiv,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_REAL_MUL")]
    PhysicalRealMul,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_REM")]
    PhysicalRem,
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_TO_STRING")]
    PhysicalToString,
    #[serde(rename = "IIR_PREDEFINED_READ")]
    Read,
    #[serde(rename = "IIR_PREDEFINED_READ_LENGTH")]
    ReadLength,
    #[serde(rename = "IIR_PREDEFINED_REAL_NOW_FUNCTION")]
    RealNowFunction,
    #[serde(rename = "IIR_PREDEFINED_REAL_PHYSICAL_MUL")]
    RealPhysicalMul,
    #[serde(rename = "IIR_PREDEFINED_REAL_TO_STRING_DIGITS")]
    RealToStringDigits,
    #[serde(rename = "IIR_PREDEFINED_REAL_TO_STRING_FORMAT")]
    RealToStringFormat,
    #[serde(rename = "IIR_PREDEFINED_RECORD_EQUALITY")]
    RecordEquality,
    #[serde(rename = "IIR_PREDEFINED_RECORD_INEQUALITY")]
    RecordInequality,
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_FINISH")]
    StdEnvFinish,
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_FINISH_STATUS")]
    StdEnvFinishStatus,
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_RESOLUTION_LIMIT")]
    StdEnvResolutionLimit,
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_STOP")]
    StdEnvStop,
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_STOP_STATUS")]
    StdEnvStopStatus,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_ARRAY_MATCH_EQUALITY")]
    StdUlogicArrayMatchEquality,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_ARRAY_MATCH_INEQUALITY")]
    StdUlogicArrayMatchInequality,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_EQUALITY")]
    StdUlogicMatchEquality,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_GREATER")]
    StdUlogicMatchGreater,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_GREATER_EQUAL")]
    StdUlogicMatchGreaterEqual,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_INEQUALITY")]
    StdUlogicMatchInequality,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_LESS")]
    StdUlogicMatchLess,
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_LESS_EQUAL")]
    StdUlogicMatchLessEqual,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_AND")]
    TfArrayAnd,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_AND")]
    TfArrayElementAnd,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_NAND")]
    TfArrayElementNand,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_NOR")]
    TfArrayElementNor,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_OR")]
    TfArrayElementOr,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_XNOR")]
    TfArrayElementXnor,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_XOR")]
    TfArrayElementXor,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_NAND")]
    TfArrayNand,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_NOR")]
    TfArrayNor,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_NOT")]
    TfArrayNot,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_OR")]
    TfArrayOr,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_XNOR")]
    TfArrayXnor,
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_XOR")]
    TfArrayXor,
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_AND")]
    TfElementArrayAnd,
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_NAND")]
    TfElementArrayNand,
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_NOR")]
    TfElementArrayNor,
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_OR")]
    TfElementArrayOr,
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_XNOR")]
    TfElementArrayXnor,
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_XOR")]
    TfElementArrayXor,
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_AND")]
    TfReductionAnd,
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_NAND")]
    TfReductionNand,
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_NOR")]
    TfReductionNor,
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_NOT")]
    TfReductionNot,
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_OR")]
    TfReductionOr,
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_XNOR")]
    TfReductionXnor,
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_XOR")]
    TfReductionXor,
    #[serde(rename = "IIR_PREDEFINED_TIME_TO_STRING_UNIT")]
    TimeToStringUnit,
    #[serde(rename = "IIR_PREDEFINED_UNIVERSAL_I_R_MUL")]
    UniversalIRMul,
    #[serde(rename = "IIR_PREDEFINED_UNIVERSAL_R_I_DIV")]
    UniversalRIDiv,
    #[serde(rename = "IIR_PREDEFINED_UNIVERSAL_R_I_MUL")]
    UniversalRIMul,
    #[serde(rename = "IIR_PREDEFINED_VECTOR_MAXIMUM")]
    VectorMaximum,
    #[serde(rename = "IIR_PREDEFINED_VECTOR_MINIMUM")]
    VectorMinimum,
    #[serde(rename = "IIR_PREDEFINED_WRITE")]
    Write,
}
