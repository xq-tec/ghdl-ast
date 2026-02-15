use std::marker::PhantomData;

use super::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Direction {
    #[serde(rename = "to")]
    To,
    #[serde(rename = "downto")]
    Downto,
}

impl Direction {
    #[must_use]
    pub fn is_ascending(&self) -> bool {
        match self {
            Direction::To => true,
            Direction::Downto => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    In,
    Out,
    InOut,
    Buffer,
    Linkage,
}

/// ```text
/// type: &physical_type_definition | &array_subtype_definition | &record_type_definition | &floating_type_definition | &enumeration_type_definition | &integer_type_definition | &integer_subtype_definition
/// base_name: &attribute_value
/// value_chain: &attribute_value
/// designated_entity: &signal_declaration | &interface_signal_declaration | &function_declaration | &entity_declaration | &subtype_declaration | &type_declaration | &package_declaration | &interface_variable_declaration | &architecture_body | &procedure_declaration | &constant_declaration | &interface_constant_declaration
/// spec_chain: &attribute_value
/// attribute_specification: &attribute_specification
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeValue {}

/// ```text
/// type: &enumeration_type_definition
/// named_entity: &enumeration_literal
/// identifier: "'?'"
/// base_name: &enumeration_literal
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterLiteral {}

/// ```text
/// choice_expression: &character_literal | &enumeration_literal | &string_literal8 | &simple_name | &integer_literal
/// choice_staticness: "local"
/// element_type_flag: bool
/// associateds: &[assertion_statement] | &[next_statement] | &[case_statement] | &[return_statement] | &[variable_assignment_statement] | &[suspend_state_statement] | &[if_statement] | &[wait_statement] | &[procedure_call_statement] | &[exit_statement] | &[simple_signal_assignment_statement] | &[for_loop_statement] | &[null_statement]
/// chain: &choice_by_range | &choice_by_others | &choice_by_expression
/// same_alternative_flag: bool
/// associated_expr: &character_literal | &enumeration_literal | &aggregate | &string_literal8 | &simple_name | &integer_literal
/// parent: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByExpression {}

/// ```text
/// right: &division_operator | &and_operator | &slice_name | &multiplication_operator | &simple_aggregate | &indexed_name | &aggregate | &xor_operator | &parenthesis_expression | &identity_operator | &pred_attribute | &physical_int_literal | &or_operator | &type_conversion | &floating_point_literal | &substraction_operator | &qualified_expression | &left_type_attribute | &nand_operator | &addition_operator | &character_literal | &null_literal | &right_array_attribute | &not_operator | &enumeration_literal | &nor_operator | &negation_operator | &simple_name | &modulus_operator | &string_literal8 | &function_call | &length_array_attribute | &high_type_attribute | &concatenation_operator | &integer_literal | &selected_element | &dereference | &absolute_operator | &remainder_operator | &val_attribute
/// type: &enumeration_type_definition
/// implementation: &function_declaration
/// left: &division_operator | &pos_attribute | &and_operator | &slice_name | &multiplication_operator | &simple_aggregate | &greater_than_operator | &succ_attribute | &exponentiation_operator | &indexed_name | &selected_name | &pred_attribute | &xor_operator | &identity_operator | &or_operator | &left_array_attribute | &physical_int_literal | &floating_point_literal | &low_array_attribute | &qualified_expression | &nand_operator | &addition_operator | &substraction_operator | &character_literal | &active_attribute | &left_type_attribute | &type_conversion | &not_operator | &enumeration_literal | &nor_operator | &modulus_operator | &right_array_attribute | &simple_name | &attribute_name | &negation_operator | &high_array_attribute | &string_literal8 | &function_call | &length_array_attribute | &concatenation_operator | &integer_literal | &selected_element | &inequality_operator | &right_type_attribute | &dereference | &absolute_operator | &remainder_operator
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EqualityOperator {}

#[derive(Debug)]
pub enum IndexList {
    Items(Vec<ExpressionNodeId>),
    Others,
}

impl IndexList {
    /// Returns the list of indices.
    ///
    /// # Panics
    ///
    /// Panics if the index list is [`Others`](IndexList::Others).
    #[must_use]
    pub fn items(&self) -> &[ExpressionNodeId] {
        #[expect(clippy::panic, reason = "panic is intentional")]
        match self {
            IndexList::Items(items) => items,
            IndexList::Others => panic!("expected list of indices, got 'others'"),
        }
    }
}

impl Serialize for IndexList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IndexList::Items(items) => items.serialize(serializer),
            IndexList::Others => serializer.serialize_str("others"),
        }
    }
}

impl<'de> Deserialize<'de> for IndexList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::SeqAccess;
        use serde::de::Visitor;
        use serde::de::value::SeqAccessDeserializer;

        struct IndexListVisitor;

        impl<'de> Visitor<'de> for IndexListVisitor {
            type Value = IndexList;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an array of node IDs or the string \"others\"")
            }

            fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let items = Vec::deserialize(SeqAccessDeserializer::new(visitor))?;
                Ok(IndexList::Items(items))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if value == "others" {
                    Ok(IndexList::Others)
                } else {
                    Err(Error::custom(format!("expected \"others\", got '{value}'")))
                }
            }
        }

        deserializer.deserialize_any(IndexListVisitor)
    }
}

/// ```text
/// sequential_statements: &[assertion_statement] | &[suspend_state_statement] | &[wait_statement] | &[return_statement] | &[variable_assignment_statement] | &[case_statement] | &[if_statement] | &[while_loop_statement] | &[procedure_call_statement] | &[exit_statement] | &[simple_signal_assignment_statement] | &[for_loop_statement] | &[null_statement]
/// suspend_flag: bool
/// condition: &active_attribute | &and_operator | &quiet_attribute | &not_operator | &equality_operator | &greater_than_operator | &enumeration_literal | &stable_attribute | &simple_name | &event_attribute | &signal_declaration | &less_than_operator | &less_than_or_equal_operator | &function_call | &selected_name | &guard_signal_declaration | &greater_than_or_equal_operator | &inequality_operator | &or_operator
/// visible_flag: bool
/// else_clause: &elsif
/// label: ""
/// covered_flag: bool
/// is_ref: bool
/// chain: &assertion_statement | &suspend_state_statement | &while_loop_statement | &next_statement | &variable_assignment_statement | &case_statement | &if_statement | &return_statement | &procedure_call_statement | &exit_statement | &simple_signal_assignment_statement | &for_loop_statement
/// parent: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IfStatement {}

/// ```text
/// condition: &function_call | &and_operator | &inequality_operator | &or_operator | &equality_operator | &greater_than_operator | &simple_name | &less_than_operator
/// sequential_statements: &[assertion_statement] | &[suspend_state_statement] | &[wait_statement] | &[return_statement] | &[variable_assignment_statement] | &[if_statement] | &[procedure_call_statement] | &[simple_signal_assignment_statement] | &[for_loop_statement] | &[null_statement]
/// is_ref: bool
/// else_clause: &elsif
/// covered_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Elsif {}

/// ```text
/// chain: &choice_by_others | &choice_by_name | &choice_by_none
/// element_type_flag: bool
/// same_alternative_flag: bool
/// associated_expr: &character_literal | &simple_aggregate | &enumeration_literal | &negation_operator | &simple_name | &string_literal8 | &aggregate | &indexed_name | &concatenation_operator | &integer_literal | &selected_element | &physical_int_literal | &floating_point_literal | &addition_operator | &physical_fp_literal
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByNone {
    #[serde(rename = "associated_expr")]
    pub expression: ExpressionNodeId,
}

/// ```text
/// covered_flag: bool
/// case_statement_alternatives: &[choice_by_range] | &[choice_by_others] | &[choice_by_expression]
/// expression: &indexed_name | &and_operator | &function_call | &simple_name | &qualified_expression | &parenthesis_expression
/// label: ""
/// chain: &assertion_statement | &procedure_call_statement | &while_loop_statement | &return_statement | &variable_assignment_statement | &if_statement
/// matching_flag: bool
/// visible_flag: bool
/// parent: int
/// suspend_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct CaseStatement {}

/// ```text
/// covered_flag: bool
/// report_expression: &string_literal8 | &function_call | &concatenation_operator
/// label: ""
/// parent: int
/// chain: &assertion_statement | &suspend_state_statement | &while_loop_statement | &return_statement | &variable_assignment_statement | &next_statement | &if_statement | &procedure_call_statement | &simple_signal_assignment_statement | &exit_statement | &for_loop_statement | &null_statement
/// assertion_condition: &active_attribute | &and_operator | &quiet_attribute | &not_operator | &equality_operator | &enumeration_literal | &greater_than_operator | &event_attribute | &simple_name | &stable_attribute | &less_than_operator | &less_than_or_equal_operator | &function_call | &xor_operator | &greater_than_or_equal_operator | &inequality_operator | &or_operator
/// visible_flag: bool
/// severity_expression: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssertionStatement {}

/// ```text
/// parent: int
/// associated_expr: &character_literal | &aggregate | &string_literal8 | &integer_literal | &physical_int_literal | &enumeration_literal | &floating_point_literal | &simple_name
/// element_type_flag: bool
/// associateds: &[assertion_statement] | &[return_statement] | &[variable_assignment_statement] | &[if_statement] | &[procedure_call_statement] | &[exit_statement] | &[simple_signal_assignment_statement] | &[null_statement]
/// same_alternative_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByOthers {}

/// ```text
/// right: &character_literal | &and_operator | &slice_name | &quiet_attribute | &event_attribute | &equality_operator | &enumeration_literal | &greater_than_operator | &not_operator | &simple_name | &less_than_operator | &string_literal8 | &less_than_or_equal_operator | &indexed_name | &xor_operator | &function_call | &greater_than_or_equal_operator | &inequality_operator | &or_operator | &selected_element | &dereference | &qualified_expression
/// type: &array_type_definition | &enumeration_type_definition
/// implementation: &function_declaration
/// left: &character_literal | &and_operator | &slice_name | &quiet_attribute | &event_attribute | &equality_operator | &enumeration_literal | &greater_than_operator | &not_operator | &simple_name | &less_than_operator | &string_literal8 | &indexed_name | &less_than_or_equal_operator | &greater_than_or_equal_operator | &inequality_operator | &or_operator | &dereference | &qualified_expression
#[derive(Debug, Deserialize, Serialize)]
pub struct AndOperator {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attribute {
    pub kind: AttributeKind,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeKind {
    Base,
    Subtype,
    Element,
    Across,
    Through,
    NatureReference,
    LeftType,
    RightType,
    HighType,
    LowType,
    AscendingType,
    Image,
    Value,
    Pos,
    Val,
    Succ,
    Pred,
    Leftof,
    Rightof,
    SignalSlew,
    QuantitySlew,
    Ramp,
    Zoh,
    Ltf,
    Ztf,
    Dot,
    Integ,
    QuantityDelayed,
    Above,
    Delayed,
    Stable,
    Quiet,
    Transaction,
    Event,
    Active,
    LastEvent,
    LastActive,
    LastValue,
    Driving,
    DrivingValue,
    Behavior,
    Structure,
    SimpleName,
    InstanceName,
    PathName,
    Converse,
    LeftArray,
    RightArray,
    HighArray,
    LowArray,
    LengthArray,
    AscendingArray,
    RangeArray,
    ReverseRangeArray,
}

/// ```text
/// left: &and_operator | &slice_name | &equality_operator | &greater_than_operator | &enumeration_literal | &simple_name | &less_than_operator | &string_literal8 | &indexed_name | &greater_than_or_equal_operator | &inequality_operator | &or_operator | &dereference | &qualified_expression
/// implementation: &function_declaration
/// type: &array_type_definition | &enumeration_type_definition
/// right: &string_literal8 | &less_than_or_equal_operator | &slice_name | &indexed_name | &greater_than_or_equal_operator | &and_operator | &inequality_operator | &equality_operator | &greater_than_operator | &enumeration_literal | &dereference | &simple_name | &qualified_expression
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct OrOperator {}

/// ```text
/// associateds: &[assertion_statement] | &[case_statement] | &[simple_signal_assignment_statement] | &[variable_assignment_statement] | &[if_statement] | &[null_statement]
/// element_type_flag: bool
/// choice_range: &range_expression
/// parent: int
/// associated_expr: &character_literal | &aggregate | &simple_name | &integer_literal
/// choice_staticness: "global" | "local"
/// same_alternative_flag: bool
/// chain: &choice_by_range | &choice_by_others | &choice_by_expression
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByRange {}

/// ```text
/// left: &character_literal | &multiplication_operator | &and_operator | &slice_name | &right_array_attribute | &simple_name | &succ_attribute | &string_literal8 | &indexed_name | &pred_attribute | &function_call | &length_array_attribute | &selected_element | &physical_int_literal | &left_array_attribute | &floating_point_literal | &dereference | &qualified_expression
/// right: &addition_operator | &character_literal | &multiplication_operator | &slice_name | &null_literal | &dereference | &not_operator | &enumeration_literal | &simple_name | &string_literal8 | &function_call | &aggregate | &indexed_name | &high_type_attribute | &length_array_attribute | &integer_literal | &physical_int_literal | &floating_point_literal | &absolute_operator | &qualified_expression | &left_type_attribute | &val_attribute
/// type: &enumeration_type_definition
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InequalityOperator {}

/// ```text
/// chain: &assertion_statement | &variable_assignment_statement | &if_statement
/// parent: int
/// label: ""
/// condition: &equality_operator | &and_operator | &simple_name | &inequality_operator
/// loop_label: &simple_name
/// covered_flag: bool
/// visible_flag: bool
/// is_ref: bool
#[derive(Debug, Deserialize, Serialize)]
pub struct ExitStatement {}

/// ```text
/// covered_flag: bool
/// visible_flag: bool
/// parent: int
/// label: ""
/// chain: &assertion_statement | &suspend_state_statement | &exit_statement | &return_statement | &variable_assignment_statement | &if_statement
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NullStatement {}

/// ```text
/// type: &enumeration_type_definition
/// implementation: &function_declaration
/// right: &physical_fp_literal | &slice_name | &integer_literal | &physical_int_literal | &right_array_attribute | &enumeration_literal | &substraction_operator | &addition_operator | &simple_name | &absolute_operator | &floating_point_literal
/// left: &division_operator | &length_array_attribute | &slice_name | &integer_literal | &physical_int_literal | &left_array_attribute | &floating_point_literal | &addition_operator | &simple_name | &negation_operator | &substraction_operator
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct LessThanOperator {}

/// ```text
/// identifier: "…"
/// parent: int
/// type: &integer_subtype_definition | &enumeration_subtype_definition
/// visible_flag: bool
/// is_ref: bool
/// has_identifier_list: bool
/// subtype_indication: &integer_subtype_definition | &enumeration_subtype_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IteratorDeclaration {}

/// ```text
/// sequential_statements: &[assertion_statement] | &[suspend_state_statement] | &[case_statement] | &[wait_statement] | &[variable_assignment_statement] | &[next_statement] | &[if_statement] | &[while_loop_statement] | &[procedure_call_statement] | &[simple_signal_assignment_statement] | &[exit_statement] | &[for_loop_statement] | &[null_statement]
/// exit_flag: bool
/// next_flag: bool
/// label: "…"
/// is_within_flag: bool
/// parameter_specification: &iterator_declaration
/// parent: int
/// suspend_flag: bool
/// covered_flag: bool
/// visible_flag: bool
/// chain: &assertion_statement | &suspend_state_statement | &case_statement | &return_statement | &variable_assignment_statement | &if_statement | &procedure_call_statement | &simple_signal_assignment_statement | &for_loop_statement
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ForLoopStatement {}

/// ```text
/// right: &exponentiation_operator | &function_call | &multiplication_operator | &integer_literal | &physical_int_literal | &type_conversion | &floating_point_literal | &substraction_operator | &simple_name | &dereference | &physical_fp_literal
/// left: &division_operator | &function_call | &multiplication_operator | &integer_literal | &physical_int_literal | &selected_element | &floating_point_literal | &addition_operator | &simple_name | &substraction_operator | &dereference
/// type: &physical_type_definition | &integer_type_definition | &floating_type_definition
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct MultiplicationOperator {}

/// ```text
/// expression: &division_operator | &string_literal8 | &aggregate | &character_literal | &parenthesis_expression | &integer_literal | &physical_int_literal | &enumeration_literal | &floating_point_literal | &simple_name
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &integer_subtype_definition
/// type_mark: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct QualifiedExpression {}

/// ```text
/// left: &dereference | &function_call | &multiplication_operator | &integer_literal | &physical_int_literal | &floating_point_literal | &substraction_operator | &qualified_expression | &simple_name | &negation_operator
/// implementation: &function_declaration
/// right: &dereference | &exponentiation_operator | &function_call | &integer_literal | &physical_int_literal | &floating_point_literal | &substraction_operator | &qualified_expression | &simple_name | &negation_operator
/// type: &physical_type_definition | &integer_type_definition | &floating_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct DivisionOperator {}

/// ```text
/// type: &access_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NullLiteral {}

/// ```text
/// type: &access_type_definition | &access_subtype_definition
/// expression: &qualified_expression
/// is_ref: bool
/// allocator_designated_type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &integer_subtype_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AllocatorByExpression {}

/// ```text
/// aggr_low_limit: &enumeration_literal | &character_literal | &simple_name | &integer_literal
/// aggr_named_flag: bool
/// aggr_min_length: int
/// aggr_dynamic_flag: bool
/// aggr_high_limit: &enumeration_literal | &character_literal | &simple_name | &integer_literal
/// sub_aggregate_info: &aggregate_info
/// aggr_others_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AggregateInfo {}

/// ```text
/// type: &physical_type_definition | &enumeration_type_definition | &integer_type_definition | &floating_type_definition
/// operand: &division_operator | &high_type_attribute | &integer_literal | &physical_int_literal | &floating_point_literal | &simple_name | &physical_fp_literal
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NegationOperator {}

/// ```text
/// operand: &character_literal | &and_operator | &slice_name | &active_attribute | &quiet_attribute | &event_attribute | &equality_operator | &enumeration_literal | &greater_than_operator | &stable_attribute | &simple_name | &string_literal8 | &function_call | &indexed_name | &xor_operator | &greater_than_or_equal_operator | &concatenation_operator | &inequality_operator | &or_operator | &dereference | &qualified_expression
/// type: &array_type_definition | &enumeration_type_definition
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NotOperator {}

/// ```text
/// type: &enumeration_type_definition
/// implementation: &function_declaration
/// left: &division_operator | &function_call | &slice_name | &integer_literal | &physical_int_literal | &floating_point_literal | &simple_name
/// right: &character_literal | &slice_name | &integer_literal | &physical_int_literal | &enumeration_literal | &floating_point_literal | &negation_operator | &simple_name | &absolute_operator
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct LessThanOrEqualOperator {}

/// ```text
/// subtype_indication: &simple_name | &array_subtype_definition
/// type: &access_type_definition
/// is_ref: bool
/// allocator_designated_type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &integer_subtype_definition
/// allocator_subtype: &simple_name | &array_subtype_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AllocatorBySubtype {}

/// ```text
/// sequential_statements: &[assertion_statement] | &[next_statement] | &[while_loop_statement] | &[case_statement] | &[variable_assignment_statement] | &[if_statement] | &[procedure_call_statement] | &[exit_statement] | &[for_loop_statement] | &[null_statement]
/// chain: &assertion_statement | &suspend_state_statement | &while_loop_statement | &variable_assignment_statement | &if_statement | &procedure_call_statement | &null_statement
/// label: "" | "l1" | "t"
/// suspend_flag: bool
/// visible_flag: bool
/// condition: &inequality_operator | &or_operator | &greater_than_operator | &not_operator | &equality_operator | &simple_name | &less_than_operator
/// parent: int
/// covered_flag: bool
/// is_ref: bool
/// exit_flag: bool
/// next_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct WhileLoopStatement {}

/// ```text
/// left: &function_call | &multiplication_operator | &slice_name | &indexed_name | &integer_literal | &physical_int_literal | &floating_point_literal | &dereference | &simple_name
/// type: &enumeration_type_definition
/// implementation: &function_declaration
/// right: &character_literal | &slice_name | &integer_literal | &right_array_attribute | &physical_int_literal | &substraction_operator | &absolute_operator | &simple_name | &floating_point_literal | &dereference
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GreaterThanOperator {}

/// ```text
/// identifier: "…"
/// is_ref: bool
/// element_position: int
/// subtype_indication: &floating_subtype_definition | &array_subtype_definition | &simple_name | &integer_subtype_definition
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition | &access_type_definition
/// visible_flag: bool
/// has_identifier_list: bool
/// parent: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ElementDeclaration {}

/// ```text
/// parent: int
/// entity_class: "signal" | "entity" | "function" | "type" | "constant" | "variable" | "package" | "subtype" | "procedure" | "architecture"
/// entity_name_list: "all" | "others" | &[simple_name]
/// static_attribute_flag: bool
/// attribute_designator: &simple_name
/// expression: &string_literal8 | &character_literal | &function_call | &aggregate | &integer_literal | &physical_int_literal | &floating_point_literal | &simple_name
/// attribute_specification_chain: &attribute_specification
/// chain: &function_declaration | &attribute_specification | &procedure_declaration | &object_alias_declaration | &constant_declaration | &anonymous_type_declaration
/// attribute_value_spec_chain: &attribute_value
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeSpecification {}

/// ```text
/// mode: "in"
/// file_logical_name: &string_literal8
/// type: &file_type_definition
/// is_ref: bool
/// has_identifier_list: bool
/// has_mode: bool
/// file_open_kind: &simple_name
/// parent: int
/// identifier: "…"
/// subtype_indication: &simple_name
/// visible_flag: bool
/// chain: &variable_declaration | &file_declaration | &procedure_declaration | &constant_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FileDeclaration {}

/// ```text
/// implementation: &function_declaration
/// left: &division_operator | &physical_int_literal | &floating_point_literal | &slice_name | &simple_name | &integer_literal
/// right: &character_literal | &slice_name | &integer_literal | &physical_int_literal | &enumeration_literal | &floating_point_literal | &absolute_operator | &simple_name
/// type: &enumeration_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GreaterThanOrEqualOperator {}

/// ```text
/// operand: &high_type_attribute | &integer_literal | &physical_int_literal | &floating_point_literal | &negation_operator | &remainder_operator | &simple_name
/// implementation: &function_declaration
/// type: &physical_type_definition | &enumeration_type_definition | &integer_type_definition | &floating_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AbsoluteOperator {}

/// ```text
/// right: &negation_operator | &simple_name | &integer_literal
/// type: &floating_type_definition | &integer_type_definition
/// left: &floating_point_literal | &addition_operator | &simple_name | &integer_literal
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ExponentiationOperator {}

/// ```text
/// collapse_signal_flag: bool
/// whole_association_flag: bool
/// actual: &indexed_name | &slice_name | &simple_name
/// actual_conversion: &function_call
/// formal_conversion: &function_call
/// chain: &association_element_by_name | &association_element_open
/// in_formal_flag: bool
/// formal: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByName {
    pub formal: Option<NameNodeId>,
    pub formal_conversion: Option<NodeId<SubprogramCall>>,
    pub actual: ExpressionNodeId,
    pub actual_conversion: Option<NodeId<SubprogramCall>>,
}

/// ```text
/// resolved_flag: bool
/// has_signal_flag: bool
/// constraint_state: "fully constrained"
/// end_has_reserved_id: bool
/// is_ref: bool
/// signal_type_flag: bool
/// type_declarator: &type_declaration
/// elements_declaration_list: &[element_declaration]
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordTypeDefinition {}

/// ```text
/// implementation: &function_declaration
/// left: &substraction_operator | &simple_name | &integer_literal
/// right: &substraction_operator | &simple_name | &integer_literal
/// type: &integer_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RemainderOperator {}

/// ```text
/// parent: int
/// identifier: "…"
/// after_drivers_flag: bool
/// seen_flag: bool
/// name: &selected_element | &indexed_name | &slice_name | &simple_name
/// type: &floating_subtype_definition | &array_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &integer_subtype_definition
/// subtype_indication: &simple_name | &array_subtype_definition
/// is_ref: bool
/// visible_flag: bool
/// chain: &variable_declaration | &attribute_declaration | &object_alias_declaration | &constant_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectAliasDeclaration {}

/// ```text
/// end_has_postponed: bool
/// stop_flag: bool
/// wait_state: "unknown"
/// has_is: bool
/// postponed_flag: bool
/// is_ref: bool
/// sensitivity_list: array | &[indexed_name] | &[selected_name] | &[slice_name] | &[guard_signal_declaration] | &[selected_element] | &[simple_name] | &[signal_declaration]
/// process_origin: &concurrent_simple_signal_assignment | &concurrent_conditional_signal_assignment | &concurrent_assertion_statement | &concurrent_selected_signal_assignment
/// label: "…"
/// chain: &for_generate_statement | &block_statement | &sensitized_process_statement | &process_statement | &component_instantiation_statement
/// seen_flag: bool
/// end_has_reserved_id: bool
/// sequential_statements: &[assertion_statement] | &[procedure_call_statement] | &[simple_signal_assignment_statement] | &[case_statement] | &[variable_assignment_statement] | &[if_statement]
/// passive_flag: bool
/// declarations: &[variable_declaration] | &[procedure_body] | &[procedure_declaration]
/// is_within_flag: bool
/// parent: int
/// visible_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SensitizedProcessStatement {
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
}

/// ```text
/// postponed_flag: bool
/// label: "" | "casa" | "cas" | "casb" | "l1" | "l2"
/// visible_flag: bool
/// chain: &concurrent_procedure_call_statement | &for_generate_statement | &block_statement | &concurrent_assertion_statement | &component_instantiation_statement | &concurrent_simple_signal_assignment | &process_statement
/// parent: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentAssertionStatement {}

/// ```text
/// is_ref: bool
/// delay_mechanism: "inertial" | "transport"
/// visible_flag: bool
/// chain: &block_statement | &concurrent_conditional_signal_assignment | &concurrent_assertion_statement | &concurrent_simple_signal_assignment | &sensitized_process_statement | &process_statement | &concurrent_selected_signal_assignment
/// parent: int
/// guarded_target_state: "false" | "true"
/// guard: &guard_signal_declaration | &signal_declaration
/// postponed_flag: bool
/// label: "a" | "csa" | "" | "c"
/// has_delay_mechanism: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentSimpleSignalAssignment {}

/// ```text
/// implementation: &function_declaration
/// left: &string_literal8 | &function_call | &slice_name | &indexed_name | &concatenation_operator | &integer_literal | &simple_aggregate | &selected_element | &dereference | &simple_name | &qualified_expression
/// right: &character_literal | &string_literal8 | &slice_name | &function_call | &indexed_name | &concatenation_operator | &integer_literal | &selected_element | &dereference | &simple_name
/// type: &array_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcatenationOperator {}

/// ```text
/// generic_map_aspects: &[association_element_by_expression] | &[association_element_open] | &[association_element_by_individual]
/// instantiated_unit: &simple_name
/// label: "…"
/// parent: int
/// component_configuration: &configuration_specification
/// configuration_specification: &configuration_specification
/// chain: &for_generate_statement | &block_statement | &sensitized_process_statement | &process_statement | &component_instantiation_statement
/// visible_flag: bool
/// has_component: bool
/// port_map_aspects: &[association_element_by_name] | &[association_element_open]
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentInstantiationStatement {
    pub label: Identifier,
    pub instantiated_unit: InstantiatedUnitNodeId,
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    #[serde(default)]
    pub port_map_aspects: Vec<AssociationElementNodeId>,
}

/// ```text
/// port_map_aspects: &[association_element_by_name]
/// entity_aspect: &entity_aspect_open | &entity_aspect_entity | &entity_aspect_configuration
/// generic_map_aspects: &[association_element_by_expression] | &[association_element_open]
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BindingIndication {}

/// ```text
/// macro_expand_flag: bool
/// ports: &[interface_signal_declaration]
/// visible_flag: bool
/// parent: int
/// identifier: "…"
/// has_is: bool
/// end_has_reserved_id: bool
/// generics: &[interface_constant_declaration]
/// chain: &configuration_specification | &signal_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentDeclaration {
    #[serde(default)]
    pub generics: Vec<NodeId<ConstantDeclaration>>,
}

/// ```text
/// same_alternative_flag: bool
/// associated_expr: &character_literal | &string_literal8 | &aggregate | &integer_literal | &selected_element | &physical_int_literal | &floating_point_literal | &simple_name
/// chain: &choice_by_others | &choice_by_name
/// element_type_flag: bool
/// choice_name: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByName {}

/// ```text
/// configuration_items: &[block_configuration] | &[component_configuration]
/// block_specification: &indexed_name | &slice_name | &simple_name
/// parent: int
/// chain: &block_configuration
/// declarations: &[use_clause]
/// prev_block_configuration: &block_configuration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockConfiguration {}

/// ```text
/// entity_name: &selected_name
/// architecture: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectEntity {
    pub entity_name: NodeId<SelectedName>,
    pub architecture: Option<NodeId<SimpleName>>,
}

/// ```text
/// binding_indication: &binding_indication
/// has_end: bool
/// parent: int
/// is_ref: bool
/// chain: &configuration_specification | &subtype_declaration | &function_declaration | &use_clause | &type_declaration | &signal_declaration
/// instantiation_list: array | &[simple_name]
/// component_name: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationSpecification {}

/// ```text
/// is_ref: bool
/// chain: &block_configuration | &component_configuration
/// binding_indication: &binding_indication
/// parent: int
/// has_end: bool
/// block_configuration: &block_configuration
/// instantiation_list: &[reference_name] | &[simple_name]
/// component_name: &reference_name | &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentConfiguration {}

/// ```text
/// right: &string_literal8 | &indexed_name | &slice_name | &simple_name | &qualified_expression
/// type: &array_type_definition | &enumeration_type_definition
/// left: &string_literal8 | &indexed_name | &slice_name | &xor_operator | &simple_name | &qualified_expression
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct XorOperator {}

/// ```text
/// type: &enumeration_type_definition | &integer_type_definition
/// left: &substraction_operator | &absolute_operator | &simple_name | &integer_literal
/// right: &substraction_operator | &simple_name | &integer_literal
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ModulusOperator {}

/// ```text
/// type: &array_type_definition | &enumeration_type_definition
/// implementation: &function_declaration
/// left: &string_literal8 | &indexed_name | &slice_name | &simple_name | &qualified_expression
/// right: &string_literal8 | &indexed_name | &slice_name | &simple_name | &qualified_expression
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NandOperator {}

/// ```text
/// right: &string_literal8 | &indexed_name | &slice_name | &simple_name | &qualified_expression
/// type: &array_type_definition | &enumeration_type_definition
/// left: &string_literal8 | &indexed_name | &slice_name | &simple_name | &qualified_expression
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NorOperator {}

/// ```text
/// concurrent_statements: &[if_generate_statement] | &[for_generate_statement] | &[block_statement] | &[sensitized_process_statement] | &[process_statement] | &[component_instantiation_statement]
/// guard_decl: &guard_signal_declaration
/// block_block_configuration: &block_configuration
/// is_within_flag: bool
/// declarations: &[component_declaration] | &[function_declaration] | &[configuration_specification] | &[subtype_declaration] | &[attribute_declaration] | &[type_declaration] | &[function_body] | &[disconnection_specification] | &[object_alias_declaration] | &[attribute_specification] | &[signal_declaration]
/// parent: int
/// attribute_value_chain: &attribute_value
/// visible_flag: bool
/// has_is: bool
/// label: "…"
/// end_has_reserved_id: bool
/// chain: &for_generate_statement | &block_statement | &sensitized_process_statement | &process_statement
/// block_header: &block_header
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockStatement {}

/// ```text
/// configuration_name: &simple_name | &selected_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectConfiguration {}

/// ```text
/// implementation: &function_declaration
/// operand: &physical_int_literal | &floating_point_literal | &simple_name | &integer_literal
/// type: &physical_type_definition | &enumeration_type_definition | &integer_type_definition | &floating_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IdentityOperator {}

/// ```text
/// referenced_name: &simple_name
/// named_entity: &component_declaration | &component_instantiation_statement
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceName {}

/// ```text
/// unit_name: &simple_name
/// type: &physical_type_definition
/// fp_value: "1.234…"
/// literal_length: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalFpLiteral {}

/// ```text
/// has_signal_flag: bool
/// resolved_flag: bool
/// is_ref: bool
/// resolution_indication: &simple_name
/// constraint_state: "fully constrained"
/// parent_type: &record_type_definition
/// subtype_type_mark: &simple_name
/// elements_declaration_list: &[element_declaration]
/// type_declarator: &subtype_declaration
/// signal_type_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordSubtypeDefinition {}

/// ```text
/// literal_origin: &and_operator | &xor_operator | &concatenation_operator | &or_operator | &nor_operator | &not_operator | &nand_operator
/// simple_aggregate_list: &[enumeration_literal] | &[integer_literal]
/// type: &array_subtype_definition
/// literal_subtype: &array_subtype_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleAggregate {}

/// ```text
/// type: &floating_subtype_definition | &enumeration_type_definition | &array_subtype_definition | &integer_subtype_definition | &array_type_definition
/// expression: &function_call | &floating_point_literal | &simple_name | &integer_literal
/// type_mark: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeConversion {}

/// ```text
/// collapse_signal_flag: bool
/// artificial_flag: bool
/// chain: &association_element_by_expression | &association_element_by_name | &association_element_open
/// in_formal_flag: bool
/// whole_association_flag: bool
/// formal: &simple_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementOpen {
    pub formal: Option<NameNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementPackage {}

/// ```text
/// has_end: bool
/// concurrent_statements: &[block_statement] | &[sensitized_process_statement] | &[process_statement] | &[component_instantiation_statement]
/// is_within_flag: bool
/// has_begin: bool
/// alternative_label: ""
/// parent: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateStatementBody {}

/// ```text
/// port_map_aspects: &[association_element_by_name] | &[association_element_open]
/// ports: &[interface_signal_declaration]
/// generic_map_aspects: &[association_element_by_expression] | &[association_element_open]
/// generics: &[interface_constant_declaration]
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockHeader {}

/// ```text
/// label: "…"
/// chain: &component_instantiation_statement | &for_generate_statement | &sensitized_process_statement | &process_statement | &if_generate_statement
/// parent: int
/// generate_statement_body: &generate_statement_body
/// end_has_reserved_id: bool
/// is_within_flag: bool
/// parameter_specification: &iterator_declaration
/// visible_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ForGenerateStatement {}

/// ```text
/// chain: &sensitized_process_statement | &process_statement | &concurrent_assertion_statement
/// label: "…"
/// covered_flag: bool
/// suspend_flag: bool
/// visible_flag: bool
/// postponed_flag: bool
/// parent: int
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentProcedureCallStatement {}

/// ```text
/// is_ref: bool
/// chain: &conditional_waveform
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalWaveform {}

/// ```text
/// covered_flag: bool
/// parent: int
/// visible_flag: bool
/// condition: &equality_operator | &and_operator | &greater_than_operator | &simple_name | &inequality_operator
/// label: ""
/// is_ref: bool
/// loop_label: &simple_name
/// chain: &variable_assignment_statement
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NextStatement {}

/// ```text
/// identifier: "guard"
/// guard_sensitivity_list: &[simple_name]
/// guard_expression: &equality_operator | &simple_name
/// is_ref: bool
/// block_statement: &block_statement
/// guarded_signal_flag: bool
/// signal_kind: "register"
/// has_active_flag: bool
/// visible_flag: bool
/// type: &enumeration_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GuardSignalDeclaration {}

/// ```text
/// choice_staticness: "local"
/// formal: &simple_name
/// in_formal_flag: bool
/// actual_type: &array_subtype_definition | &record_type_definition
/// whole_association_flag: bool
/// collapse_signal_flag: bool
/// chain: &association_element_by_expression
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByIndividual {
    pub formal: Option<NodeId<SimpleName>>,
    pub actual_type: SubtypeDefinitionNodeId,
}

/// ```text
/// type: &integer_subtype_definition | &array_subtype_definition
/// expression: &simple_name | &aggregate | &slice_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ParenthesisExpression {}

/// ```text
/// label: "l2" | "g1" | "g2" | "gif"
/// end_has_reserved_id: bool
/// condition: &simple_name | &equality_operator
/// visible_flag: bool
/// chain: &for_generate_statement | &process_statement | &if_generate_statement
/// parent: int
/// generate_statement_body: &generate_statement_body
/// is_ref: bool
/// is_within_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IfGenerateStatement {}

/// ```text
/// postponed_flag: bool
/// matching_flag: bool
/// label: ""
/// is_ref: bool
/// delay_mechanism: "transport" | "inertial"
/// parent: int
/// chain: &sensitized_process_statement | &process_statement
/// has_delay_mechanism: bool
/// guarded_target_state: "false"
/// visible_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentSelectedSignalAssignment {}

/// ```text
/// chain: &signal_declaration
/// type_mark: &simple_name
/// parent: int
/// signal_list: &[simple_name]
/// is_ref: bool
/// expression: &physical_int_literal
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct DisconnectionSpecification {}

/// ```text
/// parent: int
/// attribute_implicit_chain: &stable_attribute | &quiet_attribute
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeImplicitDeclaration {}

/// ```text
/// chain: &sensitized_process_statement | &concurrent_conditional_signal_assignment
/// delay_mechanism: "inertial" | "transport"
/// conditional_waveforms: &[conditional_waveform]
/// guarded_target_state: "false"
/// target: &simple_name
/// parent: int
/// label: ""
/// is_ref: bool
/// postponed_flag: bool
/// has_delay_mechanism: bool
/// visible_flag: bool
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentConditionalSignalAssignment {}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectOpen {}

/// ```text
/// resolved_flag: bool
/// signal_type_flag: bool
/// type_declarator: &subtype_declaration
/// parent_type: &access_type_definition
/// designated_subtype_indication: &array_subtype_definition
/// subtype_type_mark: &simple_name
/// designated_type: &floating_subtype_definition | &integer_subtype_definition | &array_subtype_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessSubtypeDefinition {}

/// ```text
/// type_declarator: &type_declaration
/// incomplete_type_ref_chain: &access_type_definition
/// resolved_flag: bool
/// signal_type_flag: bool
/// has_signal_flag: bool
/// complete_type_definition: &record_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IncompleteTypeDefinition {}

subset_declaration!(RangeConstraint RangeConstraintNodeId {
    // Ref(ElementId),
    Expression(RangeExpression),
    Attribute(Attribute),
});

subset_declaration!(InstantiatedUnit InstantiatedUnitNodeId {
    EntityAspectEntity(EntityAspectEntity),
    SimpleName(SimpleName),
    SelectedName(SelectedName),
});

/// ```text
/// resolved_flag: bool
/// signal_type_flag: bool
/// has_signal_flag: bool
/// type_declarator: &type_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Error {}

impl Error {
    /// The ID of the global error node.
    pub const GLOBAL_ID: NodeId<Self> = NodeId(nodes::IdPrimitive::new(2).unwrap(), PhantomData);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OverloadList {}
