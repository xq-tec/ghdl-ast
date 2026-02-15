use super::*;

/// ```text
/// has_is: bool
/// callees_list: &[procedure_declaration]
/// end_has_postponed: bool
/// visible_flag: bool
/// parent: int
/// passive_flag: bool
/// wait_state: "unknown"
/// end_has_reserved_id: bool
/// label: "…"
/// is_within_flag: bool
/// suspend_flag: bool
/// chain: &sensitized_process_statement | &process_statement
/// postponed_flag: bool
/// stop_flag: bool
/// process_origin: &concurrent_procedure_call_statement
/// sequential_statements: &[suspend_state_statement] | &[report_statement] | &[wait_statement] | &[assertion_statement] | &[variable_assignment_statement] | &[case_statement] | &[if_statement] | &[while_loop_statement] | &[procedure_call_statement] | &[simple_signal_assignment_statement] | &[for_loop_statement] | &[null_statement]
/// declarations: &[function_declaration] | &[suspend_state_declaration] | &[subtype_declaration] | &[anonymous_type_declaration] | &[type_declaration] | &[function_body] | &[procedure_body] | &[use_clause] | &[variable_declaration] | &[object_alias_declaration] | &[procedure_declaration] | &[file_declaration] | &[constant_declaration]
/// seen_flag: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct ProcessStatement {
    pub label: Option<Identifier>,

    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,

    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

subset_declaration!(ConcurrentStatement ConcurrentStatementNodeId {
   Process(ProcessStatement),
   Block(BlockStatement),
   SensitizedProcess(SensitizedProcessStatement),
   ForGenerate(ForGenerateStatement),
   IfGenerate(IfGenerateStatement),
   ComponentInstantiation(ComponentInstantiationStatement),
});
