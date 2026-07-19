//! PSL (Property Specification Language) nodes embedded in VHDL.
//!
//! GHDL's JSON export often omits or stubs many PSL-internal fields (`PSL-NODE`,
//! `PSL-NFA`). Structs here capture the simulation-relevant VHDL-facing fields
//! that are reliably present; treat missing optional fields as incomplete
//! export rather than absent source constructs.

use super::*;

/// PSL inherit specification attaching inherited verification content.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslInheritSpec {
    /// Name of the inherited unit / item.
    pub name: Option<NameNodeId>,
}

/// Hierarchical name binding a verification unit into the design hierarchy.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslHierarchicalName {
    /// Entity name locating the bind target.
    pub entity_name: Option<NameNodeId>,
    /// Optional architecture name when the bind names an architecture.
    pub architecture: Option<NameNodeId>,
}

/// PSL declaration (property, sequence, or related PSL declarator).
///
/// ```vhdl
/// -- PSL
/// property p_rising is always rose(clk) -> next a;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PslDeclaration {
    /// Declaration identifier.
    pub identifier: Option<Identifier>,
}

/// PSL boolean parameter of a parameterized PSL declaration.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslBooleanParameter {
    /// Parameter identifier.
    pub identifier: Option<Identifier>,
    /// Analyzed boolean type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL endpoint declaration (named endpoint for sequences).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslEndpointDeclaration {
    /// Endpoint identifier.
    pub identifier: Option<Identifier>,
    /// Analyzed type of the endpoint object.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `prev` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslPrev {
    /// Operand expression.
    pub expression: Option<ExpressionNodeId>,
    /// Optional count expression (`prev(e, n)`).
    pub count_expression: Option<ExpressionNodeId>,
    /// Explicit clock expression when present.
    pub clock_expression: Option<ExpressionNodeId>,
    /// Default clock used when no explicit clock is written.
    pub default_clock: Option<GenericNodeId>,
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `stable` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslStable {
    /// Operand expression.
    pub expression: Option<ExpressionNodeId>,
    /// Explicit clock expression when present.
    pub clock_expression: Option<ExpressionNodeId>,
    /// Default clock used when no explicit clock is written.
    pub default_clock: Option<GenericNodeId>,
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `rose` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslRose {
    /// Operand expression.
    pub expression: Option<ExpressionNodeId>,
    /// Explicit clock expression when present.
    pub clock_expression: Option<ExpressionNodeId>,
    /// Default clock used when no explicit clock is written.
    pub default_clock: Option<GenericNodeId>,
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `fell` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslFell {
    /// Operand expression.
    pub expression: Option<ExpressionNodeId>,
    /// Explicit clock expression when present.
    pub clock_expression: Option<ExpressionNodeId>,
    /// Default clock used when no explicit clock is written.
    pub default_clock: Option<GenericNodeId>,
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `onehot` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslOnehot {
    /// Operand expression.
    pub expression: Option<ExpressionNodeId>,
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `onehot0` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslOnehot0 {
    /// Operand expression.
    pub expression: Option<ExpressionNodeId>,
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Generic PSL expression node when a more specific kind is not exported.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslExpression {
    /// Analyzed type of the PSL expression.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL assert directive (`assert …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslAssertDirective {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Asserted PSL property (often a stubbed PSL node in the JSON export).
    pub psl_property: Option<GenericNodeId>,
    /// Optional report message expression.
    pub report_expression: Option<ExpressionNodeId>,
    /// Optional severity expression.
    pub severity_expression: Option<ExpressionNodeId>,
}

/// PSL assume directive (`assume …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslAssumeDirective {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Assumed PSL property (often a stubbed PSL node in the JSON export).
    pub psl_property: Option<GenericNodeId>,
}

/// PSL cover directive (`cover …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslCoverDirective {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Covered PSL sequence (often a stubbed PSL node in the JSON export).
    pub psl_sequence: Option<GenericNodeId>,
    /// Optional report message expression.
    pub report_expression: Option<ExpressionNodeId>,
}

/// PSL restrict directive (`restrict …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslRestrictDirective {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Restricted PSL sequence (often a stubbed PSL node in the JSON export).
    pub psl_sequence: Option<GenericNodeId>,
}

/// PSL default clock declaration (`default clock is …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslDefaultClock {
    /// Boolean clock expression of the default clock.
    pub psl_boolean: Option<GenericNodeId>,
}
