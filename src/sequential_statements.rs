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

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureCallStatement {
    pub procedure_call: NodeId<ProcedureCall>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureCall {
    pub prefix: PrefixNodeId,
    pub implementation: ProcedureImplementationNodeId,
    #[serde(default)]
    pub parameter_associations: Vec<AssociationElementNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReportStatement {
    pub report_expression: ExpressionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReturnStatement {
    pub expression: Option<ExpressionNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleSignalAssignmentStatement {
    pub target: ExpressionNodeId,
    #[serde(default)]
    pub waveforms: Vec<NodeId<WaveformElement>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuspendStateStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct VariableAssignmentStatement {
    pub target: ExpressionNodeId,
    pub expression: ExpressionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WaitStatement {
    #[serde(default)]
    pub sensitivity_list: Vec<ExpressionNodeId>,
    pub timeout_clause: Option<ExpressionNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IfStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Elsif {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaseStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssertionStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExitStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NullStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForLoopStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct WhileLoopStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NextStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct BreakStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct BreakElement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalSignalAssignmentStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedWaveformAssignmentStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignalForceAssignmentStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignalReleaseAssignmentStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalVariableAssignmentStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedVariableAssignmentStatement {}
