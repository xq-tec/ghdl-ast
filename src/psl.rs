use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct PslInheritSpec {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslHierarchicalName {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslDeclaration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslBooleanParameter {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslEndpointDeclaration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslPrev {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslStable {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslRose {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslFell {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslOnehot {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslOnehot0 {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslExpression {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslAssertDirective {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslAssumeDirective {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslCoverDirective {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslRestrictDirective {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PslDefaultClock {}
