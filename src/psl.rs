//! PSL (Property Specification Language) nodes embedded in VHDL.
//!
//! GHDL's JSON export often omits or stubs many PSL-internal fields (`PSL-NODE`,
//! `PSL-NFA`). Structs here capture the simulation-relevant VHDL-facing fields
//! that are reliably present; treat missing optional fields as incomplete
//! export rather than absent source constructs.

use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use serde::Deserializer;
use serde::de::Error;
use serde::de::Visitor;

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
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
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
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
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
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
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
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
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
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
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
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
    pub psl_property: Option<GenericNodeId>,
}

/// PSL cover directive (`cover …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslCoverDirective {
    /// Optional statement label.
    pub label: Option<Identifier>,
    /// Covered PSL sequence (often a stubbed PSL node in the JSON export).
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
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
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
    pub psl_sequence: Option<GenericNodeId>,
}

/// PSL default clock declaration (`default clock is …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct PslDefaultClock {
    /// Boolean clock expression of the default clock.
    ///
    /// GHDL exports the PSL node as the stub `"PSL-NODE"`, which deserializes
    /// as `None`.
    #[serde(default, deserialize_with = "deserialize_optional_psl_node")]
    pub psl_boolean: Option<GenericNodeId>,
}

/// Deserializes a PSL field exported as a node id or as a `"PSL-NODE"` / `"PSL-NFA"` stub.
///
/// Stubs, JSON `null`, and id `0` become `None`.
///
/// # Errors
///
/// Returns an error when the value is neither an integer node id nor a known stub.
fn deserialize_optional_psl_node<'de, D>(deserializer: D) -> Result<Option<GenericNodeId>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PslNodeVisitor;

    impl Visitor<'_> for PslNodeVisitor {
        type Value = Option<GenericNodeId>;

        fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
            formatter.write_str("a node id, null, or a PSL stub")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let id = u32::try_from(value).map_err(E::custom)?;
            Ok(IdPrimitive::new(id).map(GenericNodeId::from))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let id = u32::try_from(value).map_err(E::custom)?;
            Ok(IdPrimitive::new(id).map(GenericNodeId::from))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                "PSL-NODE" | "PSL-NFA" => Ok(None),
                _ => Err(E::custom(format!("unrecognized PSL node stub '{value}'"))),
            }
        }
    }

    deserializer.deserialize_any(PslNodeVisitor)
}
