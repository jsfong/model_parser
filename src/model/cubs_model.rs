use jsonpath_rust::JsonPath;
use leptos::html::A;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::char;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelResponse {
    pub data: ModelData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelData {
    pub schema_version: String,
    pub model_id: String,
    pub site_model_id: String,
    pub version: u32,
    // #[serde(alias = "cubsObjects", deserialize_with = "null_to_empty_vec")]
    #[serde(alias = "cubsObjects")]
    pub elements: Value,
    // #[serde(deserialize_with = "null_to_empty_vec")]
    pub relationships: Value,
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

//TODO refactor using trait
impl ModelData {
    pub fn execute_json_path_for_element(&self, query: &str) -> Vec<&Value> {
        println!("[execute_json_path_for_element] executing json path query: {}", query);

        let start_time = Instant::now();   
        let result = self.elements.query_with_path(query);
        let value: Vec<&Value> = match result {
            Ok(v) => {
                let values: Vec<&Value> = v.into_iter().map(|qref| qref.val()).collect();
                values
            }
            Err(_) => vec![],
        };

        //Log time
        let elapsed_time = start_time.elapsed();
        println!(
            "[Execution time] {} for query {}- {:?}",
            "json path query", 
            query,
            elapsed_time,
        );

        value
    }
}

pub fn truncate_value(values: Vec<&Value>, truncate_depth: usize) -> Vec<Value> {
    let result = values
        .iter()
        .map(|v| truncate(&v, truncate_depth, 0))
        .collect();

    result
}

fn truncate(value: &Value, max_depth: usize, current_depth: usize) -> Value {
    if current_depth >= max_depth {
        return match value {
            Value::Array(_) => Value::Null,
            Value::Object(_) => Value::Null,
            other => other.clone(),
        };
    }

    match value {
        Value::Array(arr) => {
            let new_array: Vec<Value> = arr
                .iter()
                .map(|v| truncate(v, max_depth, current_depth + 1))
                .collect();
            Value::Array(new_array)
        }

        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map {
                new_map.insert(k.clone(), truncate(v, max_depth, current_depth + 1));
            }
            Value::Object(new_map)
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_truncate() {
        // Test with a simple JSON object
        let json = r#"{"a": 1, "b": {"c": 2}}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        let result = truncate(&value, 2, 0);
        let result_string = result.to_string();
        assert_eq!(result_string, r#"{"a":1,"b":{"c":2}}"#);
    }

    #[test]
    fn test_truncate2() {
        // Test with a simple JSON object
        let json = r#"{"a": 1,"b": {"c": {"d": 2}}}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        let result = truncate(&value, 2, 0);
        let result_string = result.to_string();
        assert_eq!(result_string, r#"{"a":1,"b":{"c":null}}"#);
    }
}
