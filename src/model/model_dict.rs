use serde::de::value::Error;
use serde::Serialize;

use crate::model::cubs_model::{Element, Relationship};

use super::cubs_model::ModelData;
use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

#[derive(Debug, Serialize)]
pub struct ModelDictionary<'a> {
    pub model_id: String,
    pub version: u32,

    pub model_stats: ModelStats,
    // pub elements_stats: Option<CubsObjectReport>,
    // pub relationships_stats: Option<CubsObjectReport>,
    pub cubsobject_map: Option<ElementRefMap<'a>>,
    pub relationship_map: Option<RelationshipRefMap<'a>>,
}

#[derive(Debug, Serialize)]
pub struct ModelStats {
    elements_stats: Option<CubsObjectReport>,
    relationships_stats: Option<CubsObjectReport>,
}

#[derive(Debug, Serialize)]
pub struct CubsObjectReport {
    all_count: u32,
    by_type: HashMap<String, u32>,
    by_nature: HashMap<String, u32>,
    ordered_by_type: ElementCounts,
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

#[derive(Debug, Clone)]
pub struct ElementCount {
    element: String,
    count: u32,
}

#[derive(Debug, Clone)]
pub struct ElementCounts {
    value: Vec<ElementCount>,
}

impl Serialize for ElementCounts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        let mut map = BTreeMap::new();
        for element_count in &self.value {
            map.insert(&element_count.element, element_count.count);
        }
         map.serialize(serializer)
    }
}

impl<'a> ModelDictionary<'a> {
    pub fn from(model: &'a ModelData) -> Self {
        let start_time = Instant::now();

        /*Generate ref map */
        let type_key_getter = |e: &Element| return e.type_.clone();
        let nature_key_getter = |e: &Element| return e.nature.clone();
        let cubsobject_partitioned_map = ElementRefMap {
            type_: generate_element_ref_map(type_key_getter, &model.elements),
            nature: generate_element_ref_map(nature_key_getter, &model.elements),
        };

        /*Generate ref map for relationship */
        let rel_type_key_getter = |e: &Relationship| return e.type_.clone();
        let rel_nature_key_getter = |e: &Relationship| return e.nature.clone();
        let relationship_partitioned_map = RelationshipRefMap {
            type_: generate_relationship_ref_map(rel_type_key_getter, &model.relationships),
            nature: generate_relationship_ref_map(rel_nature_key_getter, &model.relationships),
        };

        /* Generate stats */
        // CubsObject stats
        let cubs_obj_type_cout_map: HashMap<String, u32> = cubsobject_partitioned_map
            .type_
            .iter()
            .map(|(k, v)| (k.clone(), v.len() as u32))
            .collect();
        let cubs_obj_nature_cout_map: HashMap<String, u32> = cubsobject_partitioned_map
            .nature
            .iter()
            .map(|(k, v)| (k.clone(), v.len() as u32))
            .collect();

        // Relationships stats
        let rel_type_cout_map: HashMap<String, u32> = relationship_partitioned_map
            .type_
            .iter()
            .map(|(k, v)| (k.clone(), v.len() as u32))
            .collect();
        let rel_nature_cout_map: HashMap<String, u32> = relationship_partitioned_map
            .nature
            .iter()
            .map(|(k, v)| (k.clone(), v.len() as u32))
            .collect();

        //Log time
        let elapsed_time = start_time.elapsed();
        println!(
            "[Execution time] {} - {:?}",
            "ModelDictionary::from", elapsed_time
        );

        //Ordered
        let mut ordered_by_type: Vec<ElementCount> = cubs_obj_type_cout_map
            .iter()
            .map(|(k, v)| ElementCount {
                element: k.clone(),
                count: *v,
            })
            .collect();
        ordered_by_type.sort_by(|a, b| b.count.cmp(&a.count));

        let mut rel_ordered_by_type: Vec<ElementCount> = rel_nature_cout_map
            .iter()
            .map(|(k, v)| ElementCount {
                element: k.clone(),
                count: *v,
            })
            .collect();
        rel_ordered_by_type.sort_by(|a, b| b.count.cmp(&a.count));

        // Construct output
        ModelDictionary {
            model_id: model.model_id.clone(),
            version: model.version,
            model_stats: ModelStats {
                elements_stats: Some(CubsObjectReport {
                    all_count: model.elements.len() as u32,
                    by_type: cubs_obj_type_cout_map,
                    by_nature: cubs_obj_nature_cout_map,
                    ordered_by_type: ElementCounts { value: ordered_by_type },
                }),

                relationships_stats: Some(CubsObjectReport {
                    all_count: model.relationships.len() as u32,
                    by_type: rel_type_cout_map,
                    by_nature: rel_nature_cout_map,
                    ordered_by_type: ElementCounts { value: rel_ordered_by_type },
                }),
            },
            cubsobject_map: Some(cubsobject_partitioned_map),
            relationship_map: Some(relationship_partitioned_map),
        }
    }
}

// Helper method
fn insert_count(count_map: &mut HashMap<String, u32>, key: String, value: u32) {
    match count_map.get_mut(&key) {
        Some(count) => *count += value,
        None => {
            count_map.insert(key, value);
        }
    }
}

fn generate_element_ref_map<'a, F>(
    key_getter: F,
    element: &Vec<Element>,
) -> HashMap<String, Vec<&Element>>
where
    F: Fn(&Element) -> String,
{
    let element_partitioned_map: HashMap<String, Vec<&Element>> =
        element.iter().fold(HashMap::new(), |mut acc, e| {
            let key = key_getter(e);
            let value = acc.entry(key).or_insert_with(|| Vec::new());
            value.push(&e);
            acc
        });
    element_partitioned_map
}

fn generate_relationship_ref_map<'a, F>(
    key_getter: F,
    element: &Vec<Relationship>,
) -> HashMap<String, Vec<&Relationship>>
where
    F: Fn(&Relationship) -> String,
{
    let element_partitioned_map: HashMap<String, Vec<&Relationship>> =
        element.iter().fold(HashMap::new(), |mut acc, e| {
            let key = key_getter(e);
            let value = acc.entry(key).or_insert_with(|| Vec::new());
            value.push(&e);
            acc
        });
    element_partitioned_map
}
