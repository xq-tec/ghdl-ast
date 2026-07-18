//! Choice nodes for aggregates, case statements, and selected assignments.
//!
//! GHDL uses one choice kind family for aggregates, `case` / `case?` statements,
//! `case` generate, selected signal/variable assignments, and simultaneous case.
//! Association payload is either [`associated_expr`](ChoiceByExpression::associated_expr)
//! (expression, individual association, or generate body) or
//! [`associateds`](ChoiceByExpression::associateds) (sequential statements or
//! waveform elements). Both are never set at the same time.

use super::*;

subset_declaration!(Choice ChoiceNodeId {
    ByRange(ChoiceByRange),
    ByExpression(ChoiceByExpression),
    ByOthers(ChoiceByOthers),
    ByNone(ChoiceByNone),
    ByName(ChoiceByName),
});

/// Choice by a discrete or matching expression (`when expr =>`).
///
/// For `when 1 | 2 | 3 =>`, only the first choice carries the association; the
/// following choices have [`same_alternative_flag`](Self::same_alternative_flag)
/// set and share that association.
///
/// ```vhdl
/// -- Aggregate (associated_expr = expression):
/// signal v : integer_vector(0 to 3) := (0 => 1, 1 => 2, others => 0);
///
/// -- Case statement (associateds = sequential statements):
/// case sel is
///   when 0 | 1 => report "low";   -- second choice has same_alternative_flag
///   when others => null;
/// end case;
///
/// -- Selected waveform (associateds = waveform elements):
/// with sel select
///   q <= '0' after 1 ns when 0,
///        '1' after 1 ns when 1;
///
/// -- Case generate (associated_expr = generate statement body):
/// g: case sel generate
///   when 0 =>
///     signal s : bit;
///   begin
///     ...
/// end generate;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByExpression {
    /// Whether this choice shares the previous choice's association (`|`).
    #[serde(default)]
    pub same_alternative_flag: bool,
    /// For aggregates: `true` when the associated expression is one element
    /// (not a sub-aggregate for an array/record slice).
    #[serde(default)]
    pub element_type_flag: bool,
    /// Expression that selects this alternative.
    pub choice_expression: ExpressionNodeId,
    /// Associated expression, individual association, or generate body.
    ///
    /// Used by aggregates and case generate. Absent when
    /// [`associateds`](Self::associateds) carries the association instead.
    pub associated_expr: Option<ExpressionNodeId>,
    /// Associated chain: sequential statements (case) or waveforms (selected
    /// assignment).
    ///
    /// Element kinds depend on context; IDs are stored untyped for that reason.
    /// Absent / empty when [`associated_expr`](Self::associated_expr) is used.
    #[serde(default, rename = "associateds")]
    pub associateds: Vec<GenericNodeId>,
}

/// Positional association in an aggregate (`(expr, expr, …)`).
///
/// There is no choice expression; the position in the association list is the
/// choice. [`expression`](Self::expression) is the associated value (GHDL field
/// `associated_expr`).
///
/// ```vhdl
/// constant c : integer_vector := (1, 2, 3);
/// -- three ChoiceByNone nodes, each with expression = 1 / 2 / 3
///
/// constant r : rec_t := (1, '1');  -- positional record aggregate
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByNone {
    /// Whether this choice shares the previous choice's association.
    #[serde(default)]
    pub same_alternative_flag: bool,
    /// For aggregates: `true` when the associated expression is one element.
    #[serde(default)]
    pub element_type_flag: bool,
    /// Associated expression (GHDL `associated_expr`).
    #[serde(rename = "associated_expr")]
    pub expression: ExpressionNodeId,
    /// Associated chain when used outside simple aggregates (rarely set).
    #[serde(default, rename = "associateds")]
    pub associateds: Vec<GenericNodeId>,
}

/// Catch-all choice (`when others =>`).
///
/// Must be the last alternative. Association follows the same
/// `associated_expr` / `associateds` rules as other choices.
///
/// ```vhdl
/// case sel is
///   when 0 => ...;
///   when others => null;
/// end case;
///
/// signal v : integer_vector(0 to 7) := (0 => 1, others => 0);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByOthers {
    /// Whether this choice shares the previous choice's association.
    #[serde(default)]
    pub same_alternative_flag: bool,
    /// For aggregates: `true` when the associated expression is one element.
    #[serde(default)]
    pub element_type_flag: bool,
    /// Associated expression, individual association, or generate body.
    pub associated_expr: Option<ExpressionNodeId>,
    /// Associated sequential statements or waveforms.
    #[serde(default, rename = "associateds")]
    pub associateds: Vec<GenericNodeId>,
}

/// Choice by a discrete range (`when lo to hi =>` / `when lo downto hi =>`).
///
/// ```vhdl
/// case sel is
///   when 0 to 3 => ...;
///   when 4 downto 1 => ...;  -- legal if direction matches the type
///   when others => ...;
/// end case;
///
/// signal v : integer_vector(0 to 7) := (1 to 3 => 9, others => 0);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByRange {
    /// Whether this choice shares the previous choice's association.
    #[serde(default)]
    pub same_alternative_flag: bool,
    /// For aggregates: `true` when the associated expression is one element.
    #[serde(default)]
    pub element_type_flag: bool,
    /// Discrete range of this choice.
    pub choice_range: RangeConstraintNodeId,
    /// Associated expression, individual association, or generate body.
    pub associated_expr: Option<ExpressionNodeId>,
    /// Associated sequential statements or waveforms.
    #[serde(default, rename = "associateds")]
    pub associateds: Vec<GenericNodeId>,
}

/// Named choice in a record aggregate (`when elem_name =>`).
///
/// [`choice_name`](Self::choice_name) should be a simple name of a record
/// element.
///
/// ```vhdl
/// type rec_t is record
///   a, b : integer;
/// end record;
/// constant r : rec_t := (a => 1, b => 2);
/// -- ChoiceByName.choice_name = a / b, associated_expr = 1 / 2
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByName {
    /// Whether this choice shares the previous choice's association.
    #[serde(default)]
    pub same_alternative_flag: bool,
    /// For aggregates: `true` when the associated expression is one element.
    #[serde(default)]
    pub element_type_flag: bool,
    /// Record element name selected by this choice.
    pub choice_name: NameNodeId,
    /// Associated expression for this named association.
    pub associated_expr: Option<ExpressionNodeId>,
    /// Associated chain when used outside aggregates (rarely set).
    #[serde(default, rename = "associateds")]
    pub associateds: Vec<GenericNodeId>,
}
