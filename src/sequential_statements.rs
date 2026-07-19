//! Sequential statements (LRM clause 10).
//!
//! Sequential statements appear in process bodies, subprogram bodies, and other
//! sequential regions. They are executed in order by a single process or
//! subprogram activation.

use super::*;

subset_declaration!(SequentialStatement SequentialStatementOwned SequentialStatementNodeId {
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
    Exit(ExitStatement),
    Next(NextStatement),
    Null(NullStatement),
    ConditionalSignalAssignment(ConditionalSignalAssignmentStatement),
    SelectedWaveformAssignment(SelectedWaveformAssignmentStatement),
    SignalForceAssignment(SignalForceAssignmentStatement),
    SignalReleaseAssignment(SignalReleaseAssignmentStatement),
    ConditionalVariableAssignment(ConditionalVariableAssignmentStatement),
    SelectedVariableAssignment(SelectedVariableAssignmentStatement),
    Break(BreakStatement),

    SuspendState(SuspendStateStatement),
});

/// A sequential procedure call statement.
///
/// ```vhdl
/// my_proc(a, b);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureCallStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// The procedure call itself (name, implementation, associations).
    pub procedure_call: NodeId<ProcedureCall>,
}

/// The call expression shared by sequential and concurrent procedure calls.
///
/// Holds the resolved procedure implementation and the actual parameter
/// associations after overload resolution.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureCall {
    /// Name used to denote the procedure (may be selected, indexed, etc.).
    pub prefix: PrefixNodeId,
    /// Resolved procedure declaration / interface / instantiation.
    pub implementation: ProcedureImplementationNodeId,
    /// Actual parameter associations (positional or named).
    #[serde(default)]
    pub parameter_associations: Vec<AssociationElementNodeId>,
    /// Protected-type method object when this call is a method invocation.
    pub method_object: Option<GenericNodeId>,
}

/// A `report` statement.
///
/// ```vhdl
/// report "value is " & integer'image(v) severity note;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ReportStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Message expression (typically a string).
    pub report_expression: ExpressionNodeId,
    /// Optional severity expression (`note`, `warning`, `error`, `failure`, …).
    pub severity_expression: Option<ExpressionNodeId>,
}

/// A `return` statement inside a function or procedure.
///
/// ```vhdl
/// return a + b;   -- function: expression present
/// return;         -- procedure: expression absent
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ReturnStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Returned value; absent for procedure returns.
    pub expression: Option<ExpressionNodeId>,
}

/// A simple (unconditional) sequential signal assignment.
///
/// ```vhdl
/// q <= d after 1 ns;
/// q <= transport d after 1 ns;
/// q <= reject 500 ps inertial d after 2 ns;
/// q <= unaffected;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleSignalAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Assignment target (signal name, aggregate, indexed/selected name, …).
    pub target: ExpressionNodeId,
    /// Inertial or transport delay mechanism.
    pub delay_mechanism: DelayMechanism,
    /// Optional pulse-rejection limit for inertial delay (`reject` time).
    pub reject_time_expression: Option<ExpressionNodeId>,
    /// Waveform elements; may be empty when the waveform is `unaffected`.
    #[serde(default)]
    pub waveforms: Vec<NodeId<WaveformElement>>,
}

/// GHDL-internal suspend-state machine statement inserted for simulation.
///
/// Not present in source VHDL; created by GHDL when transforming processes that
/// contain `wait` statements into a state machine.
#[derive(Debug, Deserialize, Serialize)]
pub struct SuspendStateStatement {
    /// Index of this suspend point within the process state machine.
    pub suspend_state_index: i32,
    /// Declaration of the suspend-state object.
    pub suspend_state_decl: Option<NodeId<SuspendStateDeclaration>>,
}

/// A sequential variable assignment.
///
/// ```vhdl
/// v := a + b;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct VariableAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Assignment target (variable name or selected/indexed name).
    pub target: ExpressionNodeId,
    /// Right-hand side expression.
    pub expression: ExpressionNodeId,
}

/// A `wait` statement.
///
/// Any combination of sensitivity list, condition clause, and timeout may be
/// present:
///
/// ```vhdl
/// wait;                              -- forever
/// wait on clk, rst;                  -- sensitivity only
/// wait until clk = '1';              -- condition only (implicit sensitivity)
/// wait for 10 ns;                    -- timeout only
/// wait on clk until enable = '1' for 1 us;
/// wait on all;                       -- VHDL-2008
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct WaitStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Explicit sensitivity list (`on …`), if any.
    #[serde(default)]
    pub sensitivity_list: Option<SensitivityList>,
    /// Condition clause (`until …`), if any.
    pub condition_clause: Option<ExpressionNodeId>,
    /// Timeout clause (`for …`), if any.
    pub timeout_clause: Option<ExpressionNodeId>,
}

/// An `if` statement.
///
/// The first condition and then-branch live on this node; further `elsif` /
/// `else` arms are a chain of [`Elsif`] nodes via [`else_clause`](Self::else_clause).
///
/// ```vhdl
/// if cond1 then
///   ...
/// elsif cond2 then
///   ...
/// else
///   ...
/// end if;
/// ```
///
/// An `else` arm is represented as an [`Elsif`] whose
/// [`condition`](Elsif::condition) is absent.
#[derive(Debug, Deserialize, Serialize)]
pub struct IfStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Condition of the initial `if`.
    pub condition: ExpressionNodeId,
    /// Statements in the then-branch.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
    /// First `elsif` / `else` clause, if any.
    pub else_clause: Option<NodeId<Elsif>>,
}

/// An `elsif` or `else` arm of an [`IfStatement`].
///
/// When [`condition`](Self::condition) is `None`, this node is the final `else`
/// branch. Further arms are linked via [`else_clause`](Self::else_clause).
#[derive(Debug, Deserialize, Serialize)]
pub struct Elsif {
    /// Condition of this `elsif`; absent for a plain `else`.
    pub condition: Option<ExpressionNodeId>,
    /// Statements in this arm.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
    /// Next `elsif` / `else` clause, if any.
    pub else_clause: Option<NodeId<Elsif>>,
}

/// A `case` or matching `case?` statement.
///
/// Alternatives are a chain of [`Choice`](crate::Choice) nodes. Each alternative
/// associates a choice (or several choices sharing
/// [`same_alternative_flag`](crate::ChoiceByExpression::same_alternative_flag))
/// with a sequence of statements via the choice's `associateds` chain.
///
/// ```vhdl
/// case sel is
///   when "00" | "01" => ...;
///   when "10"        => ...;
///   when others      => ...;
/// end case;
///
/// case? sel is          -- matching_flag = true (VHDL-2008)
///   when "0-" => ...;
///   when others => ...;
/// end case?;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct CaseStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Expression whose value selects an alternative.
    pub expression: ExpressionNodeId,
    /// `true` for a matching case statement (`case?`).
    pub matching_flag: bool,
    /// Alternatives as a choice chain (`when … =>`).
    #[serde(default, rename = "case_statement_alternatives")]
    pub alternatives: Vec<ChoiceNodeId>,
}

/// An `assert` statement.
///
/// ```vhdl
/// assert a = b report "mismatch" severity error;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssertionStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Condition that must hold; failure triggers the report/severity.
    pub assertion_condition: ExpressionNodeId,
    /// Optional message expression.
    pub report_expression: Option<ExpressionNodeId>,
    /// Optional severity expression.
    pub severity_expression: Option<ExpressionNodeId>,
}

/// An `exit` statement leaving a loop.
///
/// ```vhdl
/// exit;
/// exit when done;
/// exit outer when done;   -- loop_label names the target loop
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ExitStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Optional name of the loop to exit; defaults to the innermost loop.
    pub loop_label: Option<NameNodeId>,
    /// Optional condition (`exit when …`).
    pub condition: Option<ExpressionNodeId>,
}

/// A `null` statement (no operation).
///
/// ```vhdl
/// null;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NullStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
}

/// A `for` loop statement.
///
/// The iteration parameter is an [`IteratorDeclaration`] owned by this loop.
///
/// ```vhdl
/// for i in 0 to 7 loop
///   ...
/// end loop;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ForLoopStatement {
    /// Optional loop label.
    pub label: Option<Identifier>,
    /// Loop parameter declaration (`identifier in discrete_range`).
    pub parameter_specification: NodeId<IteratorDeclaration>,
    /// Loop body.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

/// A `while` loop statement.
///
/// ```vhdl
/// while not done loop
///   ...
/// end loop;
/// ```
///
/// A bare `loop … end loop` (infinite loop) is also represented as a while-loop
/// whose condition is absent or always true, depending on how GHDL parsed it;
/// typically a missing condition means an infinite loop.
#[derive(Debug, Deserialize, Serialize)]
pub struct WhileLoopStatement {
    /// Optional loop label.
    pub label: Option<Identifier>,
    /// Loop condition; may be absent for a bare infinite `loop`.
    pub condition: Option<ExpressionNodeId>,
    /// Loop body.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

/// A `next` statement continuing with the next loop iteration.
///
/// ```vhdl
/// next;
/// next when skip;
/// next outer when skip;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NextStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Optional name of the loop to continue; defaults to the innermost loop.
    pub loop_label: Option<NameNodeId>,
    /// Optional condition (`next when …`).
    pub condition: Option<ExpressionNodeId>,
}

/// An AMS `break` statement (analog simulation discontinuity).
///
/// Not used in pure digital simulation.
#[derive(Debug, Deserialize, Serialize)]
pub struct BreakStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Optional condition.
    pub condition: Option<ExpressionNodeId>,
    /// Break elements selecting quantities to break.
    #[serde(default)]
    pub break_element: Vec<NodeId<BreakElement>>,
}

/// One quantity pair in an AMS `break` statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct BreakElement {
    /// Optional selector quantity.
    pub selector_quantity: Option<ExpressionNodeId>,
    /// Quantity whose value is set.
    pub break_quantity: Option<ExpressionNodeId>,
    /// Expression assigned at the break.
    pub expression: Option<ExpressionNodeId>,
}

/// VHDL-2008 sequential conditional signal assignment.
///
/// ```vhdl
/// q <= a when sel = '1' else b;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalSignalAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Assignment target.
    pub target: ExpressionNodeId,
    /// Inertial or transport delay mechanism.
    pub delay_mechanism: DelayMechanism,
    /// Optional pulse-rejection limit.
    pub reject_time_expression: Option<ExpressionNodeId>,
    /// Conditional waveform arms.
    #[serde(default)]
    pub conditional_waveforms: Vec<NodeId<ConditionalWaveform>>,
}

/// VHDL-2008 sequential selected signal assignment.
///
/// ```vhdl
/// with sel select
///   q <= a when "00",
///        b when "01",
///        c when others;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedWaveformAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Assignment target.
    pub target: ExpressionNodeId,
    /// Selecting expression (`with … select`).
    pub expression: ExpressionNodeId,
    /// Inertial or transport delay mechanism.
    pub delay_mechanism: DelayMechanism,
    /// Optional pulse-rejection limit.
    pub reject_time_expression: Option<ExpressionNodeId>,
    /// `true` for matching selected assignment (`select?`).
    pub matching_flag: bool,
    /// Selected waveform alternatives as a choice chain.
    #[serde(default)]
    pub selected_waveforms: Vec<ChoiceNodeId>,
}

/// VHDL-2008 signal force assignment.
///
/// ```vhdl
/// s <= force in '1';
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SignalForceAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Forced signal target.
    pub target: ExpressionNodeId,
    /// Force mode (`in` = effective, `out` = driving).
    pub force_mode: Option<ForceMode>,
    /// Forced value expression.
    pub expression: ExpressionNodeId,
}

/// VHDL-2008 signal release assignment.
///
/// ```vhdl
/// s <= release out;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SignalReleaseAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Released signal target.
    pub target: ExpressionNodeId,
    /// Release mode (`in` / `out`).
    pub force_mode: Option<ForceMode>,
}

/// VHDL-2008 sequential conditional variable assignment.
///
/// ```vhdl
/// v := a when sel else b;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalVariableAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Assignment target.
    pub target: ExpressionNodeId,
    /// Conditional expression arms.
    #[serde(default)]
    pub conditional_expressions: Vec<NodeId<ConditionalExpression>>,
}

/// VHDL-2008 sequential selected variable assignment.
///
/// ```vhdl
/// with sel select
///   v := a when 0,
///        b when others;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedVariableAssignmentStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Assignment target.
    pub target: ExpressionNodeId,
    /// Selecting expression.
    pub expression: ExpressionNodeId,
    /// `true` for matching selected assignment.
    pub matching_flag: bool,
    /// Selected expression alternatives as a choice chain.
    #[serde(default)]
    pub selected_expressions: Vec<ChoiceNodeId>,
}
