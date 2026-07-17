use super::*;

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationDeclaration {
    pub identifier: Option<Identifier>,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextDeclaration {
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityDeclaration {
    pub id: NodeId<Self>,
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
    #[serde(default)]
    pub generics: Vec<InterfaceObjectDeclarationNodeId>,
    #[serde(default)]
    pub ports: Vec<NodeId<InterfaceSignalDeclaration>>,
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageDeclaration {
    pub id: NodeId<Self>,

    pub identifier: Identifier,
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageInstantiationDeclaration {
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageHeader {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InterfacePackageDeclaration {}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageBody {
    pub identifier: Identifier,
    #[serde(rename = "parent")]
    pub design_unit: NodeId<DesignUnit>,
}

subset_declaration!(ContextItem ContextItemNodeId {
    LibraryClause(LibraryClause),
    UseClause(UseClause),
});

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryClause {
    pub identifier: Identifier,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UseClause {
    pub selected_name: AnySelectedNameNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextReference {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForeignModule {}

#[derive(Debug, Deserialize, Serialize)]
pub struct VmodeDeclaration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct VpropDeclaration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct VunitDeclaration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageInstantiationBody {}
