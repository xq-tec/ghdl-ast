//! Configuration declarations and binding (LRM clause 7 / 3.4).
//!
//! These nodes describe how component instantiations are bound to design
//! entities, and how nested block/generate regions are configured.

use super::*;

/// An entity aspect that names an entity and optional architecture.
///
/// Corresponds to `entity <entity_name> [(<architecture_identifier>)]` in a
/// binding indication or direct instantiation.
///
/// ```vhdl
/// use entity work.adder(rtl);     -- architecture present
/// use entity work.adder;          -- architecture open / deferred
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectEntity {
    /// Selected name of the entity declaration (often `library.entity`).
    pub entity_name: NodeId<SelectedName>,
    /// Optional architecture simple name when an architecture is written.
    pub architecture: Option<NodeId<SimpleName>>,
}

subset_declaration!(InstantiatedUnit InstantiatedUnitNodeId {
    EntityAspectEntity(EntityAspectEntity),
    SimpleName(SimpleName),
    SelectedName(SelectedName),
});

subset_declaration!(EntityAspect EntityAspectNodeId {
    Entity(EntityAspectEntity),
    Configuration(EntityAspectConfiguration),
    Open(EntityAspectOpen),
});

subset_declaration!(ConfigurationItem ConfigurationItemNodeId {
    Block(BlockConfiguration),
    Component(ComponentConfiguration),
});

/// A binding indication that associates a component instance with a design unit.
///
/// Appears in configuration specifications, component configurations, and as
/// the implicit default binding of an unbound instantiation.
///
/// ```vhdl
/// for u1 : nand2 use entity work.nand2(rtl)
///   generic map (Tpd => 2 ns)
///   port map (i1 => a, i2 => b, o => y);
///
/// for all : adder use configuration work.adder_cfg;
/// for others : mux use open;   -- EntityAspectOpen
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BindingIndication {
    /// Entity, configuration, or `open` aspect naming the bound design unit.
    pub entity_aspect: Option<EntityAspectNodeId>,
    /// Generic map associations from the binding.
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    /// Port map associations from the binding.
    #[serde(default)]
    pub port_map_aspects: Vec<AssociationElementNodeId>,
}

/// A block configuration that configures an architecture, block, or generate region.
///
/// The block specification names the configured region; nested configuration
/// items bind components and further nested regions.
///
/// ```vhdl
/// configuration cfg of ent is
///   for rtl                          -- block_specification = architecture rtl
///     for all : u_comp use entity work.comp(rtl);
///     for gen_i                      -- nested block configuration
///       for all : cell use entity work.cell;
///     end for;
///   end for;
/// end configuration cfg;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockConfiguration {
    /// Name of the configured architecture, block label, or generate specification.
    pub block_specification: Option<NameNodeId>,
    /// Declarative items of the block configuration (typically use clauses).
    #[serde(default)]
    pub declarations: Vec<GenericNodeId>,
    /// Nested block and component configurations.
    #[serde(default)]
    pub configuration_items: Vec<ConfigurationItemNodeId>,
}

/// A component configuration that binds one or more component instantiations.
///
/// Selects instances by label list (`all` / `others` / explicit labels) and
/// supplies an optional binding indication and nested block configuration for
/// the bound architecture.
///
/// ```vhdl
/// for all : nand2 use entity work.nand2(rtl);
/// for u1, u2 : adder use entity work.adder(fast)
///   generic map (WIDTH => 8);
/// for others : mux use open;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentConfiguration {
    /// Name denoting the component declaration being configured.
    pub component_name: GenericNodeId,
    /// Instantiation labels this configuration applies to.
    pub instantiation_list: InstantiationList,
    /// Optional nested block configuration of the bound architecture.
    pub block_configuration: Option<NodeId<BlockConfiguration>>,
    /// Binding of the selected instances to an entity or configuration.
    pub binding_indication: Option<NodeId<BindingIndication>>,
}

/// An entity aspect that names a configuration declaration.
///
/// ```vhdl
/// use configuration work.adder_cfg;
/// u1: configuration work.top_cfg port map (...);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectConfiguration {
    /// Name of the configuration declaration.
    pub configuration_name: NameNodeId,
}

/// An open entity aspect (`use open`).
///
/// Leaves the binding unspecified; the instance remains unbound until a later
/// configuration supplies a concrete entity or configuration aspect.
///
/// ```vhdl
/// for u1 : nand2 use open;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectOpen {}

/// Header of a block statement declaring generics, ports, and their maps.
///
/// ```vhdl
/// b: block
///   generic (N : natural := 8);
///   generic map (N => WIDTH);
///   port (clk : in std_logic; q : out std_logic);
///   port map (clk => clk, q => q_out);
/// begin
///   ...
/// end block;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockHeader {
    /// Interface generic declarations of the block.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Interface port declarations of the block.
    #[serde(default)]
    pub ports: Vec<InterfaceObjectDeclarationNodeId>,
    /// Generic map associations connecting block generics to actuals.
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    /// Port map associations connecting block ports to actuals.
    #[serde(default)]
    pub port_map_aspects: Vec<AssociationElementNodeId>,
}
