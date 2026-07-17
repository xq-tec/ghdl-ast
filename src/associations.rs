use super::*;

subset_declaration!(AssociationElement AssociationElementNodeId {
    ByExpression(AssociationElementByExpression),
    ByIndividual(AssociationElementByIndividual),
    ByName(AssociationElementByName),
    Open(AssociationElementOpen),
    Package(AssociationElementPackage),
    Type(AssociationElementType),
    Subprogram(AssociationElementSubprogram),
    Terminal(AssociationElementTerminal),
});

subset_declaration!(AssociationConversion AssociationConversionNodeId {
    FunctionCall(FunctionCall),
    TypeConversion(TypeConversion),
});

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByExpression {
    pub formal: Option<NameNodeId>,
    pub formal_conversion: Option<AssociationConversionNodeId>,
    pub actual: ExpressionNodeId,
    pub actual_conversion: Option<AssociationConversionNodeId>,
    #[serde(rename = "inertial_flag")]
    pub inertial: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByName {
    pub formal: Option<NameNodeId>,
    pub formal_conversion: Option<AssociationConversionNodeId>,
    pub actual: ExpressionNodeId,
    pub actual_conversion: Option<AssociationConversionNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementOpen {
    pub formal: Option<NameNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementByIndividual {
    pub formal: Option<NodeId<SimpleName>>,
    pub actual_type: SubtypeDefinitionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementPackage {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementType {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementSubprogram {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssociationElementTerminal {}
