use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct WaveformElement {
    #[serde(rename = "we_value")]
    pub value: ExpressionNodeId,
    #[serde(rename = "time")]
    pub delay: Option<ExpressionNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalWaveform {}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnaffectedWaveform {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalExpression {}
