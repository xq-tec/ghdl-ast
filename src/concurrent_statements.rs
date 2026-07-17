use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessStatement {
    pub label: Option<Identifier>,

    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,

    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SensitizedProcessStatement {
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentInstantiationStatement {
    pub label: Identifier,
    pub instantiated_unit: InstantiatedUnitNodeId,
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    #[serde(default)]
    pub port_map_aspects: Vec<AssociationElementNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentAssertionStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentSimpleSignalAssignment {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentProcedureCallStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentSelectedSignalAssignment {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentConditionalSignalAssignment {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConcurrentBreakStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForGenerateStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct IfGenerateStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaseGenerateStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateStatementBody {}

#[derive(Debug, Deserialize, Serialize)]
pub struct IfGenerateElseClause {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleSimultaneousStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousNullStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousProceduralStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousCaseStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousIfStatement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimultaneousElsif {}

subset_declaration!(ConcurrentStatement ConcurrentStatementNodeId {
   Process(ProcessStatement),
   Block(BlockStatement),
   SensitizedProcess(SensitizedProcessStatement),
   ForGenerate(ForGenerateStatement),
   IfGenerate(IfGenerateStatement),
   ComponentInstantiation(ComponentInstantiationStatement),
});
