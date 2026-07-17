use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectEntity {
    pub entity_name: NodeId<SelectedName>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BindingIndication {}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockConfiguration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentConfiguration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectConfiguration {}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAspectOpen {}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockHeader {}
