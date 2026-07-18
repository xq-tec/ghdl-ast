//! Concurrent statements (LRM clause 11).
//!
//! Concurrent statements appear in entity statement parts, architecture bodies,
//! block statements, and generate statement bodies. Each denotes an independent
//! process or structural region of the design.

use super::*;

subset_declaration!(ConcurrentStatement ConcurrentStatementNodeId {
    Process(ProcessStatement),
    SensitizedProcess(SensitizedProcessStatement),
    Block(BlockStatement),
    ForGenerate(ForGenerateStatement),
    IfGenerate(IfGenerateStatement),
    CaseGenerate(CaseGenerateStatement),
    ComponentInstantiation(ComponentInstantiationStatement),
    ConcurrentAssertion(ConcurrentAssertionStatement),
    ConcurrentSimpleSignalAssignment(ConcurrentSimpleSignalAssignment),
    ConcurrentConditionalSignalAssignment(ConcurrentConditionalSignalAssignment),
    ConcurrentSelectedSignalAssignment(ConcurrentSelectedSignalAssignment),
    ConcurrentProcedureCall(ConcurrentProcedureCallStatement),
    ConcurrentBreak(ConcurrentBreakStatement),
    SimpleSimultaneous(SimpleSimultaneousStatement),
});

/// A process statement without an explicit sensitivity list.
///
/// The process suspends only at `wait` statements in its body.
///
/// ```vhdl
/// process
/// begin
///   wait until clk = '1';
///   q <= d;
/// end process;
///
/// postponed process   -- postponed_flag = true
/// begin
///   ...
/// end postponed process;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessStatement {
    /// Optional process label.
    pub label: Option<Identifier>,
    /// Whether this is a postponed process (runs in the postponed region).
    pub postponed_flag: bool,
    /// Declarative region of the process.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Sequential statement body.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

/// A process statement with an explicit sensitivity list.
///
/// Equivalent to a process whose body is wrapped in an implicit
/// `wait on <sensitivity_list>` at the end.
///
/// ```vhdl
/// process (clk, rst) begin
///   if rst = '1' then
///     q <= '0';
///   elsif rising_edge(clk) then
///     q <= d;
///   end if;
/// end process;
///
/// process (all) begin   -- SensitivityList::All (VHDL-2008)
///   q <= a and b;
/// end process;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SensitizedProcessStatement {
    /// Optional process label.
    pub label: Option<Identifier>,
    /// Whether this is a postponed process.
    pub postponed_flag: bool,
    /// Declarative region of the process.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Sequential statement body.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
    /// Sensitivity list (`process (…)`).
    pub sensitivity_list: SensitivityList,
}

/// A component instantiation statement.
///
/// Instantiates an entity, configuration, or component, connecting actuals to
/// formals via generic and port maps.
///
/// ```vhdl
/// u_add: entity work.adder(rtl)
///   generic map (WIDTH => 8)
///   port map (a => a, b => b, sum => sum);
///
/// u_c: component nand2 port map (i1, i2, o);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentInstantiationStatement {
    /// Instantiation label (required).
    pub label: Identifier,
    /// Instantiated unit (entity aspect, component name, …).
    pub instantiated_unit: InstantiatedUnitNodeId,
    /// Generic map associations.
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    /// Port map associations.
    #[serde(default)]
    pub port_map_aspects: Vec<AssociationElementNodeId>,
}

/// A block statement: a nested concurrent region with optional guard,
/// generics, and ports.
///
/// ```vhdl
/// b: block (oe = '1') is          -- guard expression → guard_decl
///   port (…);
///   port map (…);
/// begin
///   ...
/// end block;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockStatement {
    /// Block label.
    pub label: Option<Identifier>,
    /// Implicit GUARD signal declaration when a guard expression is present.
    pub guard_decl: Option<NodeId<GuardSignalDeclaration>>,
    /// Optional block header (generics/ports and their maps).
    pub block_header: Option<NodeId<BlockHeader>>,
    /// Declarative region.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Concurrent statement part.
    #[serde(default)]
    pub concurrent_statements: Vec<ConcurrentStatementNodeId>,
    /// Associated block configuration, if any.
    #[serde(rename = "block_block_configuration")]
    pub block_configuration: Option<NodeId<BlockConfiguration>>,
}

/// A concurrent assertion statement.
///
/// ```vhdl
/// assert not (oe = '1' and we = '1')
///   report "bus fight" severity error;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentAssertionStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Whether this is a postponed concurrent assertion.
    pub postponed_flag: bool,
    /// Asserted condition.
    pub assertion_condition: ExpressionNodeId,
    /// Optional message.
    pub report_expression: Option<ExpressionNodeId>,
    /// Optional severity.
    pub severity_expression: Option<ExpressionNodeId>,
}

/// A concurrent simple signal assignment.
///
/// Elaborates to an equivalent sensitized process driving the target.
///
/// ```vhdl
/// q <= d after 1 ns;
/// q <= guarded d;           -- uses block GUARD
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentSimpleSignalAssignment {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Whether this is a postponed assignment.
    pub postponed_flag: bool,
    /// Assignment target.
    ///
    /// May be absent on incomplete / placeholder nodes in library packages.
    pub target: Option<ExpressionNodeId>,
    /// Inertial or transport delay mechanism.
    pub delay_mechanism: DelayMechanism,
    /// Optional pulse-rejection limit.
    pub reject_time_expression: Option<ExpressionNodeId>,
    /// Waveform elements.
    #[serde(default)]
    pub waveforms: Vec<NodeId<WaveformElement>>,
    /// GUARD signal when this is a guarded assignment inside a block.
    pub guard: Option<ExpressionNodeId>,
}

/// A concurrent procedure call statement.
///
/// ```vhdl
/// check_parity(data, parity);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentProcedureCallStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Whether this is a postponed call.
    pub postponed_flag: bool,
    /// The procedure call.
    pub procedure_call: NodeId<ProcedureCall>,
}

/// A concurrent selected signal assignment.
///
/// ```vhdl
/// with sel select
///   q <= a when "00",
///        b when "01",
///        c when others;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentSelectedSignalAssignment {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Whether this is a postponed assignment.
    pub postponed_flag: bool,
    /// Assignment target.
    pub target: ExpressionNodeId,
    /// Selecting expression.
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
    /// GUARD signal when this is a guarded assignment.
    pub guard: Option<ExpressionNodeId>,
}

/// A concurrent conditional signal assignment.
///
/// ```vhdl
/// q <= a when sel = '1' else
///      b when en  = '1' else
///      '0';
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentConditionalSignalAssignment {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Whether this is a postponed assignment.
    pub postponed_flag: bool,
    /// Assignment target.
    pub target: ExpressionNodeId,
    /// Inertial or transport delay mechanism.
    pub delay_mechanism: DelayMechanism,
    /// Optional pulse-rejection limit.
    pub reject_time_expression: Option<ExpressionNodeId>,
    /// Conditional waveform arms.
    #[serde(default)]
    pub conditional_waveforms: Vec<NodeId<ConditionalWaveform>>,
    /// GUARD signal when this is a guarded assignment.
    pub guard: Option<ExpressionNodeId>,
}

/// A concurrent AMS `break` statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentBreakStatement {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Whether this is postponed.
    pub postponed_flag: bool,
    /// Optional condition.
    pub condition: Option<ExpressionNodeId>,
    /// Optional sensitivity list.
    #[serde(default)]
    pub sensitivity_list: Option<SensitivityList>,
    /// Break elements.
    #[serde(default)]
    pub break_element: Vec<NodeId<BreakElement>>,
}

/// A `for … generate` statement.
///
/// ```vhdl
/// gen: for i in 0 to WIDTH-1 generate
///   ...
/// end generate;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ForGenerateStatement {
    /// Generate label.
    pub label: Option<Identifier>,
    /// Generate parameter (`identifier in discrete_range`).
    pub parameter_specification: NodeId<IteratorDeclaration>,
    /// Body of the generate statement.
    pub generate_statement_body: NodeId<GenerateStatementBody>,
}

/// An `if … generate` statement.
///
/// Else/elsif arms are a chain of [`IfGenerateElseClause`] nodes.
///
/// ```vhdl
/// g: if FAST generate
///   ...
/// else generate
///   ...
/// end generate;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IfGenerateStatement {
    /// Generate label.
    pub label: Option<Identifier>,
    /// Condition of the initial `if`.
    pub condition: ExpressionNodeId,
    /// Body for the true branch.
    pub generate_statement_body: NodeId<GenerateStatementBody>,
    /// First `elsif` / `else` generate clause, if any.
    pub generate_else_clause: Option<NodeId<IfGenerateElseClause>>,
}

/// A `case … generate` statement.
///
/// Alternatives are a choice chain; each choice associates a
/// [`GenerateStatementBody`] via `associated_expr` (aliased as associated
/// block in GHDL).
///
/// ```vhdl
/// g: case sel generate
///   when 0 => ...
///   when others => ...
/// end generate;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct CaseGenerateStatement {
    /// Generate label.
    pub label: Option<Identifier>,
    /// Selecting expression.
    pub expression: ExpressionNodeId,
    /// Alternatives as a choice chain.
    #[serde(default, rename = "case_statement_alternatives")]
    pub alternatives: Vec<ChoiceNodeId>,
}

/// Body of a generate alternative: declarations and concurrent statements.
///
/// ```vhdl
/// for i in 0 to 3 generate
///   signal s : bit;          -- declarations
/// begin
///   u: entity work.cell port map (...);  -- concurrent_statements
/// end generate;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateStatementBody {
    /// Alternative label (`when alt_label: …` in case generate).
    pub alternative_label: Option<Identifier>,
    /// Declarative region.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Concurrent statement part.
    #[serde(default)]
    pub concurrent_statements: Vec<ConcurrentStatementNodeId>,
    /// Associated block configuration, if any.
    pub generate_block_configuration: Option<NodeId<BlockConfiguration>>,
}

/// An `elsif` / `else` arm of an [`IfGenerateStatement`].
///
/// When [`condition`](Self::condition) is `None`, this is the final `else
/// generate` arm.
#[derive(Debug, Deserialize, Serialize)]
pub struct IfGenerateElseClause {
    /// Condition of this `elsif generate`; absent for `else generate`.
    pub condition: Option<ExpressionNodeId>,
    /// Body for this arm.
    pub generate_statement_body: NodeId<GenerateStatementBody>,
    /// Next else/elsif clause, if any.
    pub generate_else_clause: Option<NodeId<IfGenerateElseClause>>,
}

/// A simple simultaneous statement (AMS: `quantity == expression`).
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleSimultaneousStatement {
    /// Optional label.
    pub label: Option<Identifier>,
    /// Left-hand side quantity/expression.
    pub simultaneous_left: Option<ExpressionNodeId>,
    /// Right-hand side expression.
    pub simultaneous_right: Option<ExpressionNodeId>,
    /// Optional tolerance aspect.
    pub tolerance: Option<ExpressionNodeId>,
}

/// An AMS simultaneous `null` statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousNullStatement {
    /// Optional label.
    pub label: Option<Identifier>,
}

/// An AMS simultaneous procedural statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousProceduralStatement {
    /// Optional label.
    pub label: Option<Identifier>,
    /// Declarative region.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Sequential statements in the procedural region.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

/// An AMS simultaneous case statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousCaseStatement {
    /// Optional label.
    pub label: Option<Identifier>,
    /// Selecting expression.
    pub expression: ExpressionNodeId,
    /// Alternatives as a choice chain.
    #[serde(default, rename = "case_statement_alternatives")]
    pub alternatives: Vec<ChoiceNodeId>,
}

/// An AMS simultaneous if statement.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousIfStatement {
    /// Optional label.
    pub label: Option<Identifier>,
    /// Condition.
    pub condition: ExpressionNodeId,
    /// Simultaneous statements in the then-branch.
    #[serde(default)]
    pub simultaneous_statements: Vec<NodeId<SimpleSimultaneousStatement>>,
    /// Else/elsif chain.
    pub else_clause: Option<NodeId<SimultaneousElsif>>,
}

/// An AMS simultaneous elsif/else arm.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousElsif {
    /// Condition; absent for else.
    pub condition: Option<ExpressionNodeId>,
    /// Simultaneous statements in this arm.
    #[serde(default)]
    pub simultaneous_statements: Vec<NodeId<SimpleSimultaneousStatement>>,
    /// Next else/elsif clause.
    pub else_clause: Option<NodeId<SimultaneousElsif>>,
}
