use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelResponse {
    pub data: ModelData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelData {
    pub schema_version: String,
    pub model_id: String,
    pub site_model_id: String,
    pub version: u32,
    #[serde(alias = "cubsObjects", deserialize_with = "null_to_empty_vec")]
    pub elements: Vec<Element>,
    #[serde(deserialize_with = "null_to_empty_vec")]
    pub relationships: Vec<Relationship>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub id: String,
    #[serde(alias = "type")]
    pub type_: String,
    pub nature: String,
    #[serde(default)]
    pub name: String,
    pub version: u32,
    #[serde(default)]
    pub dynamic_facets: HashMap<String, serde_json::Value>,
    pub facets: HashMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub core_facets: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    #[serde(alias = "type")]
    pub type_: String,
    pub nature: String,
    #[serde(default)]
    pub name: String,
    pub version: u32,
    #[serde(default)]
    pub dynamic_facets: HashMap<String, serde_json::Value>,
    pub facets: HashMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub core_facets: HashMap<String, serde_json::Value>,
}
fn null_to_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
