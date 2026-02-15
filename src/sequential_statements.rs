use super::*;

subset_declaration!(SequentialStatement SequentialStatementNodeId {
    ProcedureCall(ProcedureCallStatement),
    Report(ReportStatement),
    Assert(AssertionStatement),
    Return(ReturnStatement),
    SimpleSignalAssignment(SimpleSignalAssignmentStatement),
    VariableAssignment(VariableAssignmentStatement),
    Wait(WaitStatement),
    If(IfStatement),
    ForLoop(ForLoopStatement),
    Case(CaseStatement),
    While(WhileLoopStatement),

    SuspendState(SuspendStateStatement),
});

subset_declaration!(AssociationElement AssociationElementNodeId {
    ByExpression(AssociationElementByExpression),
    ByIndividual(AssociationElementByIndividual),
    ByName(AssociationElementByName),
    Open(AssociationElementOpen),
});

/// ```text
/// suspend_flag: bool
/// covered_flag: bool
/// parent: int
/// procedure_call: &procedure_call
/// label: ""
/// visible_flag: bool
/// chain: &assertion_statement | &suspend_state_statement | &return_statement | &variable_assignment_statement | &if_statement | &procedure_call_statement | &exit_statement | &simple_signal_assignment_statement | &for_loop_statement
/// ```
#[derive(Debug, Deserialize)]
pub struct ProcedureCallStatement {
    pub procedure_call: NodeId<SubprogramCall>,
}

/// ```text
/// whole_association_flag: bool
/// collapse_signal_flag: bool
/// actual: &floating_point_literal | &character_literal | &multiplication_operator | &slice_name | &simple_aggregate | &right_array_attribute | &enumeration_literal | &simple_name | &string_literal8 | &aggregate | &indexed_name | &selected_name | &function_call | &concatenation_operator | &integer_literal | &physical_int_literal | &left_array_attribute | &selected_element | &substraction_operator | &dereference | &addition_operator | &qualified_expression | &val_attribute
/// chain: &association_element_by_expression | &association_element_open | &association_element_by_individual
/// in_formal_flag: bool
/// formal: &selected_element | &indexed_name | &simple_name
/// formal_conversion: &function_call
/// actual_conversion: &function_call
/// inertial_flag: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct AssociationElementByExpression {
    pub formal: Option<NameNodeId>,
    pub actual: ExpressionNodeId,
    #[serde(rename = "inertial_flag")]
    pub inertial: bool,
}

/// ```text
/// visible_flag: bool
/// label: ""
/// chain: &suspend_state_statement
/// report_expression: &string_literal8
/// parent: int
/// covered_flag: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct ReportStatement {
    pub report_expression: ExpressionNodeId,
}

/// ```text
/// label: ""
/// parent: int
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition | &array_type_definition
/// covered_flag: bool
/// visible_flag: bool
/// chain: &variable_assignment_statement | &assertion_statement
/// expression: &character_literal | &multiplication_operator | &and_operator | &enumeration_literal | &simple_name | &exponentiation_operator | &aggregate | &indexed_name | &function_call | &string_literal8 | &length_array_attribute | &integer_literal | &physical_int_literal | &selected_element | &substraction_operator | &floating_point_literal | &addition_operator | &qualified_expression | &val_attribute
/// ```
#[derive(Debug, Deserialize)]
pub struct ReturnStatement {
    pub expression: Option<ExpressionNodeId>,
}

/// ```text
/// is_ref: bool
/// covered_flag: bool
/// guarded_target_state: "unknown" | "true" | "false"
/// has_delay_mechanism: bool
/// chain: &suspend_state_statement | &assertion_statement | &simple_signal_assignment_statement | &variable_assignment_statement | &if_statement
/// visible_flag: bool
/// parent: int
/// delay_mechanism: "inertial" | "transport"
/// waveforms: &[waveform_element]
/// label: ""
/// target: &selected_element | &indexed_name | &selected_name | &aggregate | &simple_name
/// ```
#[derive(Debug, Deserialize)]
pub struct SimpleSignalAssignmentStatement {
    pub target: ExpressionNodeId,
    #[serde(default)]
    pub waveforms: Vec<NodeId<WaveformElement>>,
}

/// ```text
/// suspend_state_decl: &suspend_state_declaration
/// covered_flag: bool
/// suspend_state_index: int
/// parent: int
/// suspend_state_chain: &suspend_state_statement
/// chain: &wait_statement | &procedure_call_statement
/// ```
#[derive(Debug, Deserialize)]
pub struct SuspendStateStatement {}

/// ```text
/// expression: &division_operator | &pos_attribute | &multiplication_operator | &slice_name | &and_operator | &equality_operator | &greater_than_operator | &succ_attribute | &exponentiation_operator | &aggregate | &indexed_name | &xor_operator | &identity_operator | &or_operator | &physical_int_literal | &left_array_attribute | &floating_point_literal | &addition_operator | &qualified_expression | &substraction_operator | &nand_operator | &type_conversion | &character_literal | &null_literal | &allocator_by_subtype | &enumeration_literal | &allocator_by_expression | &not_operator | &negation_operator | &simple_name | &nor_operator | &less_than_operator | &modulus_operator | &string_literal8 | &function_call | &length_array_attribute | &less_than_or_equal_operator | &greater_than_or_equal_operator | &concatenation_operator | &integer_literal | &selected_element | &inequality_operator | &high_type_attribute | &dereference | &absolute_operator | &val_attribute
/// is_ref: bool
/// parent: int
/// target: &indexed_name | &slice_name | &selected_name | &aggregate | &selected_element | &dereference | &simple_name
/// chain: &assertion_statement | &suspend_state_statement | &while_loop_statement | &return_statement | &variable_assignment_statement | &case_statement | &if_statement | &next_statement | &procedure_call_statement | &exit_statement | &simple_signal_assignment_statement | &for_loop_statement | &null_statement
/// visible_flag: bool
/// label: ""
/// covered_flag: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct VariableAssignmentStatement {
    pub target: ExpressionNodeId,
    pub expression: ExpressionNodeId,
}

/// ```text
/// chain: &assertion_statement | &suspend_state_statement | &variable_assignment_statement | &if_statement | &procedure_call_statement | &simple_signal_assignment_statement | &for_loop_statement
/// covered_flag: bool
/// parent: int
/// is_ref: bool
/// timeout_clause: &physical_int_literal | &multiplication_operator | &substraction_operator | &simple_name
/// visible_flag: bool
/// sensitivity_list: array | &[simple_name] | &[indexed_name]
/// condition_clause: &or_operator | &equality_operator | &simple_name | &greater_than_or_equal_operator
/// label: ""
/// ```
#[derive(Debug, Deserialize)]
pub struct WaitStatement {
    #[serde(default)]
    pub sensitivity_list: Vec<ExpressionNodeId>,
    pub timeout_clause: Option<ExpressionNodeId>,
}

/// ```text
/// chain: &waveform_element
/// we_value: &nand_operator | &character_literal | &multiplication_operator | &slice_name | &null_literal | &and_operator | &selected_element | &not_operator | &enumeration_literal | &simple_name | &succ_attribute | &less_than_operator | &string_literal8 | &aggregate | &selected_name | &function_call | &indexed_name | &concatenation_operator | &integer_literal | &physical_int_literal | &exponentiation_operator | &type_conversion | &floating_point_literal | &addition_operator | &qualified_expression | &absolute_operator | &substraction_operator
/// time: &physical_int_literal | &multiplication_operator | &simple_name
/// ```
#[derive(Debug, Deserialize)]
pub struct WaveformElement {
    #[serde(rename = "we_value")]
    pub value: ExpressionNodeId,
    #[serde(rename = "time")]
    pub delay: Option<ExpressionNodeId>,
}
