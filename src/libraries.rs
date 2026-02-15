use super::*;

/// ```text
/// library_declaration
///
/// vendor_library_flag: bool
/// visible_flag: bool
/// identifier: "std" | "work"
/// library_directory: "…"
/// elab_flag: bool
/// chain: &library_declaration
/// design_files: &[design_file]
/// ```
#[derive(Debug, Deserialize)]
pub struct Library {
    pub identifier: Identifier,
    #[serde(default)]
    pub design_files: Vec<NodeId<DesignFile>>,
}

impl Library {
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

/// ```text
/// library: &library_declaration
/// design_file_directory: "/home/xq/apps/ghdl/lib/ghdl/std/v93/" | ""
/// design_file_filename: "…"
/// last_design_unit: &design_unit
/// elab_flag: bool
/// design_units: &[design_unit]
/// chain: &design_file
/// ```
#[derive(Debug, Deserialize)]
pub struct DesignFile {
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

/// ```text
/// hash_chain: &design_unit
/// design_unit_source_col: int
/// design_unit_source_line: int
/// library_unit: &package_declaration | &architecture_body | &configuration_declaration | &package_body | &entity_declaration
/// elab_flag: bool
/// chain: &design_unit
/// configuration_done_flag: bool
/// dependence_list: array | &[design_unit] | &[entity_aspect_entity]
/// identifier: "…"
/// configuration_mark_flag: bool
/// design_file: &design_file
/// context_items: &[use_clause] | &[library_clause]
/// ```
#[derive(Debug, Deserialize)]
pub struct DesignUnit {
    pub library_unit: LibraryUnitNodeId,
    pub design_file: NodeId<DesignFile>,
    #[serde(default)]
    pub context_items: Vec<ContextItemNodeId>,
}

subset_declaration!(LibraryUnit LibraryUnitNodeId {
    ConfigurationDeclaration(ConfigurationDeclaration),
    ContextDeclaration(ContextDeclaration),
    EntityDeclaration(EntityDeclaration),
    PackageDeclaration(PackageDeclaration),
    PackageInstantiationDeclaration(PackageInstantiationDeclaration),

    ArchitectureBody(ArchitectureBody),
    PackageBody(PackageBody),
});

/// ```text
/// end_has_reserved_id: bool
/// parent: int
/// identifier: "…"
/// is_within_flag: bool
/// entity_name: &simple_name
/// block_configuration: &block_configuration
/// visible_flag: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct ConfigurationDeclaration {
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

#[derive(Debug, Deserialize)]
pub struct ContextDeclaration {
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

/// ```text
/// visible_flag: bool
/// ports: &[interface_signal_declaration]
/// is_within_flag: bool
/// concurrent_statements: &[sensitized_process_statement] | &[process_statement]
/// identifier: "…"
/// has_begin: bool
/// macro_expand_flag: bool
/// end_has_reserved_id: bool
/// parent: int
/// declarations: &[attribute_declaration] | &[procedure_body] | &[anonymous_type_declaration] | &[use_clause] | &[disconnection_specification] | &[object_alias_declaration] | &[constant_declaration] | &[signal_declaration] | &[function_declaration] | &[subtype_declaration] | &[function_body] | &[type_declaration] | &[attribute_specification] | &[procedure_declaration]
/// generics: &[interface_constant_declaration]
/// attribute_value_chain: &attribute_value
/// ```
#[derive(Debug, Deserialize)]
pub struct EntityDeclaration {
    pub id: NodeId<Self>,
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    #[serde(default)]
    pub generics: Vec<NodeId<InterfaceConstantDeclaration>>,
    #[serde(default)]
    pub ports: Vec<NodeId<InterfaceSignalDeclaration>>,
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
}

/// ```text
/// macro_expand_flag: bool
/// parent: int
/// declarations: &[component_declaration] | &[function_declaration] | &[subtype_declaration] | &[attribute_declaration] | &[anonymous_type_declaration] | &[type_declaration] | &[use_clause] | &[file_declaration] | &[procedure_declaration] | &[attribute_specification] | &[constant_declaration] | &[signal_declaration]
/// end_has_reserved_id: bool
/// visible_flag: bool
/// attribute_value_chain: &attribute_value
/// need_instance_bodies: bool
/// need_body: bool
/// identifier: "…"
/// is_within_flag: bool
/// package_body: &package_body
/// ```
#[derive(Debug, Deserialize)]
pub struct PackageDeclaration {
    pub id: NodeId<Self>,

    pub identifier: Identifier,
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

#[derive(Debug, Deserialize)]
pub struct PackageInstantiationDeclaration {
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

#[derive(Debug, Deserialize)]
pub struct PackageHeader {}

#[derive(Debug, Deserialize)]
pub struct InterfacePackageDeclaration {}

/// ```text
/// identifier: "…"
/// entity_name: &simple_name
/// is_within_flag: bool
/// declarations: &[configuration_specification] | &[procedure_body] | &[attribute_declaration] | &[anonymous_type_declaration] | &[use_clause] | &[disconnection_specification] | &[object_alias_declaration] | &[constant_declaration] | &[signal_declaration] | &[component_declaration] | &[function_declaration] | &[subtype_declaration] | &[type_declaration] | &[function_body] | &[attribute_specification] | &[procedure_declaration] | &[attribute_implicit_declaration]
/// foreign_flag: bool
/// end_has_reserved_id: bool
/// parent: int
/// visible_flag: bool
/// attribute_value_chain: &attribute_value
/// concurrent_statements: &[if_generate_statement] | &[for_generate_statement] | &[block_statement] | &[sensitized_process_statement] | &[process_statement] | &[component_instantiation_statement]
/// macro_expand_flag: bool
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct ArchitectureBody {
    pub identifier: Identifier,
    pub entity_name: NodeId<SimpleName>,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    #[serde(default)]
    pub concurrent_statements: Vec<ConcurrentStatementNodeId>,
}

/// ```text
/// declarations: &[function_declaration] | &[subtype_declaration] | &[procedure_body] | &[anonymous_type_declaration] | &[function_body] | &[type_declaration] | &[procedure_declaration] | &[attribute_specification] | &[constant_declaration]
/// parent: int
/// end_has_reserved_id: bool
/// attribute_value_chain: &attribute_value
/// package: &package_declaration
/// identifier: "…"
/// ```
#[derive(Debug, Deserialize)]
pub struct PackageBody {
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

subset_declaration!(ContextItem ContextItemNodeId {
    LibraryClause(LibraryClause),
    UseClause(UseClause),
});

/// ```text
/// chain: &library_clause | &use_clause
/// library_declaration: &library_declaration
/// identifier: "std" | "work"
/// parent: int
/// has_identifier_list: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct LibraryClause {
    pub identifier: Identifier,
}

/// ```text
/// parent: int
/// selected_name: &selected_by_all_name | &selected_name
/// use_clause_chain: &use_clause
/// chain: &function_declaration | &variable_declaration | &use_clause | &procedure_declaration
/// ```
#[derive(Debug, Deserialize)]
pub struct UseClause {
    pub selected_name: AnySelectedNameNodeId,
}
