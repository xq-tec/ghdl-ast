//! Libraries, design units, and context items (LRM clauses 3 and 13).
//!
//! A [`Library`] owns [`DesignFile`]s of [`DesignUnit`]s. Primary units
//! (entity, package, configuration, context, package instantiation) and
//! secondary units (architecture, package body) are the [`LibraryUnit`]
//! variants.

use super::*;

/// A design library containing analyzed design files.
#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    /// Library logical name (`work`, `ieee`, …).
    pub identifier: Identifier,
    /// Design files analyzed into this library.
    #[serde(default)]
    pub design_files: Vec<NodeId<DesignFile>>,
}

impl Library {
    /// Iterates over all library units contained in this library's design files.
    pub fn library_units_iter<'ast>(
        &self,
        ast: &'ast Ast,
    ) -> impl Iterator<Item = (LibraryUnitNodeId, LibraryUnit<'ast>)> + use<'ast, '_> {
        self.design_files_iter(ast)
            .flat_map(|(_, design_file)| design_file.design_units_iter(ast))
            .map(|(_, design_unit)| (design_unit.library_unit, design_unit.library_unit.get(ast)))
    }

    fn design_files_iter<'ast, 'this>(
        &'this self,
        ast: &'ast Ast,
    ) -> impl Iterator<Item = (NodeId<DesignFile>, &'ast DesignFile)> + use<'ast, 'this> {
        self.design_files.iter().map(|&id| (id, id.get(ast)))
    }
}

/// One analyzed source file and the design units it contains.
#[derive(Debug, Deserialize, Serialize)]
pub struct DesignFile {
    /// Design units declared in this file (in analysis order).
    #[serde(default)]
    pub design_units: Vec<NodeId<DesignUnit>>,
}

impl DesignFile {
    fn design_units_iter<'ast>(
        &self,
        ast: &'ast Ast,
    ) -> impl Iterator<Item = (NodeId<DesignUnit>, &'ast DesignUnit)> + use<'ast, '_> {
        self.design_units.iter().map(|&id| (id, id.get(ast)))
    }
}

/// Wrapper around a library unit plus its context clause.
///
/// Context items (`library` / `use` / `context` references) apply to
/// [`library_unit`](Self::library_unit).
#[derive(Debug, Deserialize, Serialize)]
pub struct DesignUnit {
    /// Primary or secondary unit of this design unit.
    pub library_unit: LibraryUnitNodeId,
    /// Owning design file.
    pub design_file: NodeId<DesignFile>,
    /// Context clause items preceding the library unit.
    #[serde(default)]
    pub context_items: Vec<ContextItemNodeId>,
}

subset_declaration!(LibraryUnit LibraryUnitOwned LibraryUnitNodeId {
    ConfigurationDeclaration(ConfigurationDeclaration),
    ContextDeclaration(ContextDeclaration),
    EntityDeclaration(EntityDeclaration),
    PackageDeclaration(PackageDeclaration),
    PackageInstantiationDeclaration(PackageInstantiationDeclaration),

    ArchitectureBody(ArchitectureBody),
    PackageBody(PackageBody),
});

/// Configuration declaration (`configuration … of … is …`).
///
/// Binds component instantiations of an entity's architecture hierarchy.
///
/// ```vhdl
/// configuration cfg of ent is
///   for rtl
///     for all : u_comp use entity work.comp(rtl);
///   end for;
/// end configuration cfg;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationDeclaration {
    /// Configuration identifier.
    pub identifier: Option<Identifier>,
    /// Owning design unit.
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    /// Entity this configuration applies to.
    pub entity_name: Option<NameNodeId>,
    /// Declarative items of the configuration.
    #[serde(default)]
    pub declarations: Vec<GenericNodeId>,
    /// Top-level block configuration.
    pub block_configuration: Option<NodeId<BlockConfiguration>>,
}

/// Context declaration (`context … is … end context`).
///
/// Packages a reusable set of library/use/context clauses.
///
/// ```vhdl
/// context my_ctx is
///   library ieee;
///   use ieee.std_logic_1164.all;
/// end context my_ctx;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ContextDeclaration {
    /// Context identifier.
    pub identifier: Identifier,
    /// Owning design unit.
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    /// Nested context items (library / use / context references).
    #[serde(default)]
    pub context_items: Vec<ContextItemNodeId>,
}

/// Entity declaration (`entity … is … end`).
///
/// Declares the interface (generics and ports) and optional entity declarative
/// / concurrent statement regions of a design entity.
///
/// ```vhdl
/// entity adder is
///   generic (WIDTH : positive := 8);
///   port (
///     a, b : in  std_logic_vector(WIDTH - 1 downto 0);
///     sum  : out std_logic_vector(WIDTH - 1 downto 0)
///   );
/// end entity adder;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityDeclaration {
    /// Node ID of this entity (exported by GHDL for self-reference).
    pub id: NodeId<Self>,
    /// Entity identifier.
    pub identifier: Identifier,
    /// Owning design unit.
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    /// Generic interface list.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Port interface list.
    #[serde(default)]
    pub ports: Vec<PortInterfaceDeclarationNodeId>,
    /// Entity declarative region.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
}

/// Package declaration (`package … is … end`).
///
/// ```vhdl
/// package util_pkg is
///   constant WIDTH : natural := 8;
///   function clog2(n : natural) return natural;
/// end package util_pkg;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageDeclaration {
    /// Node ID of this package (exported by GHDL for self-reference).
    pub id: NodeId<Self>,

    /// Package identifier.
    pub identifier: Identifier,
    /// Declarations in the package declarative part.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Owning design unit.
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    /// Optional generic header (`generic (…); generic map (…);`).
    pub package_header: Option<NodeId<PackageHeader>>,
    /// Corresponding package body, when analyzed.
    pub package_body: Option<NodeId<PackageBody>>,
}

/// Package instantiation declaration (`package … is new …`).
///
/// ```vhdl
/// package int_queues is new work.queue_pkg
///   generic map (element_t => integer);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageInstantiationDeclaration {
    /// Instantiated package identifier.
    pub identifier: Identifier,
    /// Owning design unit.
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    /// Name of the uninstantiated package being instantiated.
    pub uninstantiated_package_name: Option<NameNodeId>,
    /// Declaration of the uninstantiated package, when resolved.
    pub uninstantiated_package_decl: Option<NodeId<PackageDeclaration>>,
    /// Formal generic declarations of the instance (copied / linked).
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Generic map associations.
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    /// Declarations visible through the instance.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Instantiation body, when present.
    pub instance_package_body: Option<NodeId<PackageInstantiationBody>>,
}

/// Generic header of a package declaration (`generic (…);` and optional map).
///
/// ```vhdl
/// package gen_pkg is
///   generic (
///     type element_t;
///     WIDTH : natural := 8
///   );
///   -- …
/// end package;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageHeader {
    /// Package generic interface list.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Optional default generic map on the package header.
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
}

/// Interface package declaration in a generic list (`generic (package … is new …)`).
///
/// ```vhdl
/// generic (
///   package queues is new work.queue_pkg generic map (<>)
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfacePackageDeclaration {
    /// Interface package identifier.
    pub identifier: Option<Identifier>,
    /// Uninstantiated package name.
    pub uninstantiated_package_name: Option<NameNodeId>,
    /// Uninstantiated package declaration, when resolved.
    pub uninstantiated_package_decl: Option<NodeId<PackageDeclaration>>,
    /// Formal generics of the interface package.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Generic map aspect (`generic map (<>)` or explicit associations).
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    /// Declarations made visible through the interface package.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Associated actual package after generic map elaboration.
    pub associated_package: Option<GenericNodeId>,
}

/// Architecture body (`architecture … of … is … begin … end`).
///
/// ```vhdl
/// architecture rtl of adder is
///   signal carry : std_logic;
/// begin
///   -- concurrent statements
/// end architecture rtl;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArchitectureBody {
    /// Architecture identifier.
    pub identifier: Identifier,
    /// Name of the entity this architecture implements.
    pub entity_name: NameNodeId,
    /// Owning design unit.
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    /// Architecture declarative region.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Concurrent statements of the architecture.
    #[serde(default)]
    pub concurrent_statements: Vec<ConcurrentStatementNodeId>,
}

/// Package body (`package body … is … end`).
///
/// Completes deferred constants and subprogram bodies declared in the package.
///
/// ```vhdl
/// package body util_pkg is
///   function clog2(n : natural) return natural is
///   begin
///     -- …
///   end function;
/// end package body util_pkg;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageBody {
    /// Package body identifier (matches the package name).
    pub identifier: Identifier,
    /// Owning design unit.
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    /// Declarations and subprogram bodies in the package body.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Corresponding package declaration.
    pub package: Option<NodeId<PackageDeclaration>>,
}

subset_declaration!(ContextItem ContextItemOwned ContextItemNodeId {
    LibraryClause(LibraryClause),
    UseClause(UseClause),
    ContextReference(ContextReference),
});

/// Library clause (`library ieee;`).
#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryClause {
    /// Library logical name being made visible.
    pub identifier: Identifier,
}

/// Use clause (`use ieee.std_logic_1164.all;`).
#[derive(Debug, Deserialize, Serialize)]
pub struct UseClause {
    /// Selected name of the package / declarations being used.
    pub selected_name: AnySelectedNameNodeId,
}

/// Context reference (`context work.my_ctx;`).
///
/// Inserts the referenced context declaration's items into the current context
/// clause.
///
/// ```vhdl
/// context work.project_ctx;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ContextReference {
    /// Selected name of the context declaration.
    pub selected_name: Option<AnySelectedNameNodeId>,
}

/// Foreign module wrapper used by GHDL's foreign/VPI elaboration path.
///
/// Appears for modules imported from outside VHDL; generics/ports mirror the
/// foreign interface GHDL exposes to the design.
#[derive(Debug, Deserialize, Serialize)]
pub struct ForeignModule {
    /// Module identifier.
    pub identifier: Option<Identifier>,
    /// Owning design unit, when present.
    pub design_unit: Option<NodeId<DesignUnit>>,
    /// Generic interface list.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Port interface list.
    #[serde(default)]
    pub ports: Vec<PortInterfaceDeclarationNodeId>,
}

/// VHDL verification mode unit (`vmode …`).
///
/// Part of the VHDL verification constructs (related to PSL/vunit tooling).
/// Many nested PSL fields are not fully exported by GHDL JSON.
#[derive(Debug, Deserialize, Serialize)]
pub struct VmodeDeclaration {
    /// Verification mode identifier.
    pub identifier: Option<Identifier>,
    /// Hierarchical name binding this unit into the design.
    pub hierarchical_name: Option<GenericNodeId>,
    /// Items declared inside the vmode.
    #[serde(default)]
    pub vunit_items: Vec<GenericNodeId>,
}

/// VHDL verification property unit (`vprop …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct VpropDeclaration {
    /// Verification property identifier.
    pub identifier: Option<Identifier>,
    /// Hierarchical name binding this unit into the design.
    pub hierarchical_name: Option<GenericNodeId>,
    /// Items declared inside the vprop.
    #[serde(default)]
    pub vunit_items: Vec<GenericNodeId>,
}

/// VHDL verification unit (`vunit …`).
///
/// ```vhdl
/// vunit my_vu (ent(rtl)) {
///   -- verification items / PSL
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct VunitDeclaration {
    /// Verification unit identifier.
    pub identifier: Option<Identifier>,
    /// Hierarchical name binding this unit into the design.
    pub hierarchical_name: Option<GenericNodeId>,
    /// Items declared inside the vunit.
    #[serde(default)]
    pub vunit_items: Vec<GenericNodeId>,
}

/// Body belonging to a package instantiation.
///
/// Holds the instance-specific declarations that complete an instantiated
/// package when an immediate or separate body is required.
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageInstantiationBody {
    /// Identifier of the instantiation body.
    pub identifier: Option<Identifier>,
    /// Declarations in the instantiation body.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Corresponding package instantiation declaration.
    pub package: Option<NodeId<PackageInstantiationDeclaration>>,
}
