//! Association elements for port, generic, and parameter maps (LRM clause 6.5.7).
//!
//! An association element connects a formal (interface object or interface
//! package/type/subprogram/terminal) to an actual. Named association writes the
//! formal explicitly; positional association leaves `formal` empty and relies on
//! declaration order.

use super::*;

subset_declaration!(AssociationElement AssociationElementOwned AssociationElementNodeId {
    ByExpression(AssociationElementByExpression),
    ByIndividual(AssociationElementByIndividual),
    ByName(AssociationElementByName),
    Open(AssociationElementOpen),
    Package(AssociationElementPackage),
    Type(AssociationElementType),
    Subprogram(AssociationElementSubprogram),
    Terminal(AssociationElementTerminal),
});

subset_declaration!(AssociationConversion AssociationConversionOwned AssociationConversionNodeId {
    FunctionCall(FunctionCall),
    TypeConversion(TypeConversion),
});

/// An association element whose actual is an expression.
///
/// This is the common form for port maps, generic maps, and subprogram call
/// parameter associations when the actual is an expression (including a name).
///
/// Named vs positional association:
///
/// ```vhdl
/// -- positional (formal is None): order matches the interface list
/// port map (clk, rst, d, q);
///
/// -- named (formal is Some(...))
/// port map (
///   clk => clock,
///   rst => reset,
///   d   => data_in,
///   q   => data_out
/// );
/// ```
///
/// Conversion functions on formal and/or actual:
///
/// ```vhdl
/// -- actual conversion: convert the actual before connecting to the formal
/// port map (a => to_bit(sl_a));
///
/// -- formal conversion: convert the formal's view of the actual
/// port map (to_stdulogic(b) => sl_b);
///
/// -- both sides (allowed when modes and types permit)
/// port map (to_bit(f) => to_stdulogic(a));
/// ```
///
/// Inertial actuals (VHDL-2008) force inertial association of a signal actual:
///
/// ```vhdl
/// port map (y => inertial pulse);   -- inertial_flag = true
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByExpression {
    /// Formal name when this is a named association; `None` when positional.
    pub formal: Option<NameNodeId>,
    /// Optional conversion applied to the formal side of the association.
    pub formal_conversion: Option<AssociationConversionNodeId>,
    /// Actual expression (may itself be a name, literal, call, …).
    pub actual: ExpressionNodeId,
    /// Optional conversion applied to the actual before association.
    pub actual_conversion: Option<AssociationConversionNodeId>,
    /// Whether the actual is associated with the `inertial` keyword.
    #[serde(rename = "inertial_flag")]
    pub inertial: bool,
}

/// An association element whose actual is a name (signal, variable, …).
///
/// Semantically similar to [`AssociationElementByExpression`], but GHDL uses
/// this kind when the actual is recognized as a name rather than a general
/// expression. Conversion functions may still appear on either side.
///
/// ```vhdl
/// port map (q => data_out);          -- named, actual is a signal name
/// port map (to_bit(a) => bit_a);     -- with formal conversion
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByName {
    /// Formal name when this is a named association; `None` when positional.
    pub formal: Option<NameNodeId>,
    /// Optional conversion applied to the formal side of the association.
    pub formal_conversion: Option<AssociationConversionNodeId>,
    /// Actual name (exported as an expression node).
    pub actual: ExpressionNodeId,
    /// Optional conversion applied to the actual before association.
    pub actual_conversion: Option<AssociationConversionNodeId>,
}

/// An association element that leaves the formal unconnected (`open`).
///
/// Valid for formals that have a default, for `out`/`inout` formals that need
/// no reader, and in other cases allowed by the LRM association rules.
///
/// ```vhdl
/// port map (clk => clk, unused => open);
/// generic map (WIDTH => 8, DEBUG => open);  -- if DEBUG has a default
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementOpen {
    /// Formal name when this is a named association; `None` when positional.
    pub formal: Option<NameNodeId>,
}

/// An association element that individually associates elements of a composite formal.
///
/// Used when an array or record formal is associated element-by-element rather
/// than as a whole. The formal names the composite interface object; the actual
/// type records the type of the individually associated actual. Nested element
/// associations appear in [`individual_associations`](Self::individual_associations)
/// when GHDL builds that chain.
///
/// ```vhdl
/// -- array formal associated by index
/// port map (
///   data(7 downto 4) => nibble_h,
///   data(3 downto 0) => nibble_l
/// );
///
/// -- record formal associated by element name
/// port map (
///   bus.addr => addr,
///   bus.data => data,
///   bus.we   => we
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByIndividual {
    /// Formal naming the composite interface object being individually associated.
    pub formal: Option<NameNodeId>,
    /// Type of the actual used for the individual association.
    pub actual_type: Option<SubtypeDefinitionNodeId>,
    /// Actual type definition when distinct from [`actual_type`](Self::actual_type).
    pub actual_type_definition: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Nested individual associations for elements of the formal.
    #[serde(default)]
    pub individual_associations: Vec<AssociationElementNodeId>,
}

/// An association element for an interface package generic.
///
/// Associates a formal interface package with an actual package (typically an
/// instantiated or declared package name).
///
/// ```vhdl
/// generic (
///   package fixed_pkg is new ieee.fixed_generic_pkg
///     generic map (<>)
/// );
/// ...
/// generic map (
///   fixed_pkg => ieee.fixed_pkg
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementPackage {
    /// Formal interface package name when named; `None` when positional.
    pub formal: Option<NameNodeId>,
    /// Actual package name associated with the formal.
    pub actual: NameNodeId,
}

/// An association element for an interface type generic.
///
/// Associates a formal interface type with an actual type mark. Related
/// interface subprograms of the formal type may be associated via
/// `subprogram_associations`.
///
/// ```vhdl
/// generic (
///   type elem_t;
///   function "=" (l, r : elem_t) return boolean is <>
/// );
/// ...
/// generic map (
///   elem_t => integer
///   -- matching "=" may be filled implicitly or listed explicitly
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementType {
    /// Formal interface type name when named; `None` when positional.
    pub formal: Option<NameNodeId>,
    /// Actual type mark associated with the formal.
    pub actual: NameNodeId,
    /// Associations for interface subprograms belonging to the formal type.
    #[serde(default)]
    pub subprogram_associations: Vec<AssociationElementNodeId>,
    /// Analyzed actual type of the association.
    pub actual_type: SubtypeDefinitionNodeId,
}

/// An association element for an interface subprogram generic.
///
/// Associates a formal interface function or procedure with an actual
/// subprogram name (or implicit default `<>` resolved by analysis).
///
/// ```vhdl
/// generic (
///   function to_string (v : T) return string is <>
/// );
/// ...
/// generic map (
///   to_string => my_to_string
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementSubprogram {
    /// Formal interface subprogram name when named; `None` when positional.
    pub formal: Option<NameNodeId>,
    /// Actual subprogram name associated with the formal.
    pub actual: NameNodeId,
}

/// An association element for an interface terminal (VHDL-AMS).
///
/// Associates a formal interface terminal with an actual terminal name.
///
/// ```vhdl
/// port (
///   terminal p, n : electrical
/// );
/// ...
/// port map (
///   p => anode,
///   n => cathode
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementTerminal {
    /// Formal interface terminal name when named; `None` when positional.
    pub formal: Option<NameNodeId>,
    /// Actual terminal name associated with the formal.
    pub actual: NameNodeId,
}
