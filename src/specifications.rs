//! Specifications: attributes, configuration, disconnection, step limit (LRM §7).
//!
//! Specifications associate additional information with already declared
//! entities (attribute values, component bindings, disconnect times, AMS step
//! limits).

use compact_str::CompactString;

use super::*;

/// Attribute specification (`attribute … of … : … is …`).
///
/// Creates [`AttributeValue`](crate::AttributeValue) nodes for each designated
/// entity in [`entity_name_list`](Self::entity_name_list).
///
/// ```vhdl
/// attribute keep : boolean;
/// attribute keep of clk, rst : signal is true;
/// attribute keep of all : signal is false;
/// attribute keep of others : signal is true;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeSpecification {
    /// Entity class token (`signal`, `entity`, `function`, …).
    pub entity_class: Option<CompactString>,
    /// Designators this specification applies to (`all` / `others` / names).
    pub entity_name_list: Option<InstantiationList>,
    /// Attribute value expression.
    pub expression: Option<ExpressionNodeId>,
    /// Attribute designator (name of the attribute declaration).
    pub attribute_designator: Option<NameNodeId>,
    /// First attribute value generated for the designated entities.
    ///
    /// Despite the GHDL field name `*_Chain`, this is exported as a single node
    /// ID (the chain head), not as a JSON array.
    pub attribute_value_specs: Option<NodeId<AttributeValue>>,
}

/// Configuration specification (`for … : … use …`).
///
/// Binds component instantiations in the enclosing declarative region without a
/// separate configuration declaration.
///
/// ```vhdl
/// for all : nand2 use entity work.nand2(rtl);
/// for u1, u2 : adder use entity work.adder(fast)
///   generic map (WIDTH => 8);
/// for others : mux use open;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationSpecification {
    /// Component name being configured.
    pub component_name: Option<GenericNodeId>,
    /// Instantiation labels this specification applies to.
    pub instantiation_list: Option<InstantiationList>,
    /// Binding indication (`use entity …` / `use configuration …` / `use open`).
    pub binding_indication: Option<NodeId<BindingIndication>>,
}

/// Disconnection specification (`disconnect … : … after …`).
///
/// Gives the time delay used when guarded drivers of the listed signals
/// disconnect.
///
/// ```vhdl
/// disconnect all : std_logic after 5 ns;
/// disconnect s1, s2 : std_logic after 2 ns;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct DisconnectionSpecification {
    /// Signals this disconnect time applies to (`all` / `others` / names).
    pub signal_list: Option<InstantiationList>,
    /// Type mark restricting which signals are affected.
    pub type_mark: Option<NameNodeId>,
    /// Disconnect time expression (`after …`).
    pub expression: Option<ExpressionNodeId>,
}

/// Step-limit specification for AMS quantities (`limit … : … with …`).
///
/// ```vhdl
/// limit q : real with 1.0e-6;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct StepLimitSpecification {
    /// Quantities this limit applies to (`all` / `others` / names).
    pub quantity_list: Option<InstantiationList>,
    /// Type mark restricting which quantities are affected.
    pub type_mark: Option<NameNodeId>,
    /// Step-limit expression.
    pub expression: Option<ExpressionNodeId>,
}

/// Entity-class entry in a group template or attribute specification context.
///
/// Stores the entity class token (`signal`, `entity`, `<>` / box, …).
///
/// ```vhdl
/// group pin2pin is (signal, signal);
/// -- two EntityClass nodes with entity_class = "signal"
///
/// attribute keep of others : signal is true;
/// -- AttributeSpecification.entity_class = "signal"
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityClass {
    /// Entity class token image (`signal`, `entity`, `function`, `<>`, …).
    pub entity_class: Option<CompactString>,
}
