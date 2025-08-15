use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

use crate::model::cubs_model::{Element, Relationship};
use super::cubs_model::ModelData;


#[derive(Debug, Serialize)]
pub struct ModelDictionary {
    pub model_id: String,
    pub version: u32,
    pub model_stats: ModelStats,
    // TODO for faster ref
    // pub cubsobject_map: Option<ElementRefMap<'a>>,
    // pub relationship_map: Option<RelationshipRefMap<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelStats {
    pub elements_stats: Option<CubsObjectReport>,
    pub relationships_stats: Option<CubsObjectReport>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CubsObjectReport {
    pub all_count: u32,
    pub by_type: ElementCounts,
    pub by_nature: ElementCounts,
}

#[derive(Debug, Serialize)]
pub struct ElementRefMap<'a> {
    pub type_: HashMap<String, Vec<&'a Element>>,
    pub nature: HashMap<String, Vec<&'a Element>>,
}

#[derive(Debug, Serialize)]
pub struct RelationshipRefMap<'a> {
    pub type_: HashMap<String, Vec<&'a Relationship>>,
    pub nature: HashMap<String, Vec<&'a Relationship>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementCount {
    pub element: String,
    pub count: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ElementCounts {
   pub value: Vec<ElementCount>,
}

impl ModelDictionary {
    pub fn from(model: &ModelData) -> Self {
        let start_time = Instant::now();

        /* Generate stats */
        let element_type_count =
            generate_array_field_count(&model.elements, "type").unwrap_or_default();
        let element_nature_count =
            generate_array_field_count(&model.elements, "nature").unwrap_or_default();
        let rel_type_count =
            generate_array_field_count(&model.relationships, "type").unwrap_or_default();
        let rel_nature_count =
            generate_array_field_count(&model.relationships, "nature").unwrap_or_default();

        //Log time
        let elapsed_time = start_time.elapsed();
        println!(
            "[Execution time] {} - {:?}",
            "ModelDictionary::from", elapsed_time
        );

        // Construct output
        ModelDictionary {
            model_id: model.model_id.clone(),
            version: model.version,
            model_stats: ModelStats {
                elements_stats: Some(CubsObjectReport {
                    all_count: get_json_array_len(&model.elements),
                    by_type: element_type_count,
                    by_nature: element_nature_count,
                }),

                relationships_stats: Some(CubsObjectReport {
                    all_count: get_json_array_len(&model.relationships),
                    by_type: rel_type_count,
                    by_nature: rel_nature_count,
                }),
            },
        }
    }
}

// Helper method
// fn insert_count(count_map: &mut HashMap<String, u32>, key: String, value: u32) {
//     match count_map.get_mut(&key) {
//         Some(count) => *count += value,
//         None => {
//             count_map.insert(key, value);
//         }
//     }
// }

// fn generate_element_ref_map<'a, F>(
//     key_getter: F,
//     element: &Vec<Element>,
// ) -> HashMap<String, Vec<&Element>>
// where
//     F: Fn(&Element) -> String,
// {
//     let element_partitioned_map: HashMap<String, Vec<&Element>> =
//         element.iter().fold(HashMap::new(), |mut acc, e| {
//             let key = key_getter(e);
//             let value = acc.entry(key).or_insert_with(|| Vec::new());
//             value.push(&e);
//             acc
//         });
//     element_partitioned_map
// }

// fn generate_relationship_ref_map<'a, F>(
//     key_getter: F,
//     element: &Vec<Relationship>,
// ) -> HashMap<String, Vec<&Relationship>>
// where
//     F: Fn(&Relationship) -> String,
// {
//     let element_partitioned_map: HashMap<String, Vec<&Relationship>> =
//         element.iter().fold(HashMap::new(), |mut acc, e| {
//             let key = key_getter(e);
//             let value = acc.entry(key).or_insert_with(|| Vec::new());
//             value.push(&e);
//             acc
//         });
//     element_partitioned_map
// }

pub fn generate_array_field_count(value: &Value, field_name: &str) -> Option<ElementCounts> {
    let array = value.as_array()?;

    let mut type_counts: HashMap<String, u32> = HashMap::new();

    for element in array {
        if let Some(type_value) = element.get(field_name) {
            if let Some(type_str) = type_value.as_str() {
                *type_counts.entry(type_str.to_owned()).or_insert(0) += 1;
            }
        }
    }

    if type_counts.is_empty() {
        return None;
    }

    let mut counts: Vec<ElementCount> = type_counts
        .into_iter()
        .map(|(element, count)| ElementCount { element, count })
        .collect();
    counts.sort_by(|a, b| b.count.cmp(&a.count));

    Some(ElementCounts { value: counts })
}

fn get_json_array_len(value: &Value) -> u32 {
    if let Some(array) = value.as_array() {
        array.len() as u32
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_array_with_types() {
        let json = json!([
            {"type": "cube", "id": 1},
            {"type": "sphere", "id": 2},
            {"type": "cube", "id": 3},
            {"type": "cube", "id": 4}
        ]);

        let result = generate_array_field_count(&json, "type").unwrap();

        assert_eq!(result.value.len(), 2);
        assert_eq!(result.value[0].element, "cube");
        assert_eq!(result.value[0].count, 3);
        assert_eq!(result.value[1].element, "sphere");
        assert_eq!(result.value[1].count, 1);
    }

    #[test]
    fn test_empty_array() {
        let json = json!([]);
        let result = generate_array_field_count(&json, "type");
        assert!(result.is_none());
    }

    #[test]
    fn test_non_array_input() {
        let json = json!({"not": "array"});
        let result = generate_array_field_count(&json, "type");
        assert!(result.is_none());
    }

    #[test]
    fn test_array_without_type_fields() {
        let json = json!([{"id": 1}, {"name": "test"}]);
        let result = generate_array_field_count(&json, "type");
        assert!(result.is_none());
    }
}
