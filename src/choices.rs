use super::*;

subset_declaration!(Choice ChoiceNodeId {
    ByRange(ChoiceByRange),
    ByExpression(ChoiceByExpression),
    ByOthers(ChoiceByOthers),
    ByNone(ChoiceByNone),
    ByName(ChoiceByName),
});

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByExpression {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByNone {
    #[serde(rename = "associated_expr")]
    pub expression: ExpressionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByOthers {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByRange {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceByName {}
