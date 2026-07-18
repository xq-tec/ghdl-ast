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
    /// Name path locating the bind target.
    pub name: Option<NameNodeId>,
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
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `stable` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslStable {
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `rose` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslRose {
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `fell` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslFell {
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `onehot` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslOnehot {
    /// Analyzed result type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// PSL `onehot0` built-in application.
#[derive(Debug, Deserialize, Serialize)]
pub struct PslOnehot0 {
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
    /// Optional label / identifier.
    pub identifier: Option<Identifier>,
}

/// PSL assume directive (`assume …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslAssumeDirective {
    /// Optional label / identifier.
    pub identifier: Option<Identifier>,
}

/// PSL cover directive (`cover …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslCoverDirective {
    /// Optional label / identifier.
    pub identifier: Option<Identifier>,
}

/// PSL restrict directive (`restrict …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslRestrictDirective {
    /// Optional label / identifier.
    pub identifier: Option<Identifier>,
}

/// PSL default clock declaration (`default clock is …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslDefaultClock {
    /// Optional identifier when the default clock is named.
    pub identifier: Option<Identifier>,
}
