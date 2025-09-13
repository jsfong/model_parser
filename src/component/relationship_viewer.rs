use crate::model::{cubs_model::Element, model_error::ModelError};
use leptos::prelude::*;

//TODO Relationship output struct
// TODO generic
struct RelationshipDetail<'a> {
    parents: Vec<&'a Element>,
    target: &'a Element,
    childs: Vec<ElementWithChild<'a>>,    
}

struct ElementWithChild<'a> {
    element: &'a Element,
    child: Vec<ElementWithChild<'a>>,
}

#[component]
pub fn RelationshipViewer(
    model_id: ReadSignal<String>,
    selected_version: ReadSignal<String>,
    selected_object_id: ReadSignal<String>,
) -> impl IntoView {
    // Create a resource of relationship detail
    // Resource trigger the fetcher when id changed
    let async_relationship_detail = Resource::new(
        move || {
            (
                model_id.get(),
                selected_version.get(),
                selected_object_id.get(),
            )
        },
        |(model_id, version, id)| async move { get_relationship_detail(model_id, version, id).await },
    );

    let async_relationship_detail_result = move || {
        let result = async_relationship_detail.get();

        match result {
            Some(result) => match result {
                Ok(r) => r,
                Err(e) => e.to_string(),
            },
            None => "Loading...".to_string(),
        }
    };

    view! { <span>"Relationship viewer for "{async_relationship_detail_result}</span> }
}

#[server(GetRelationship, "/api")]
pub async fn get_relationship_detail(
    model_id: String,
    version: String,
    id: String,
) -> Result<String, ModelError> {
    use crate::model::app_state;
    use crate::model::element_parser::ElementConnectorBuilder;
    use actix_web::web::Data;
    use leptos::logging::log;
    use leptos_actix::*;
    use std::time::Instant;

    // Validate input
    if model_id.is_empty() || id.is_empty() {
        return Err(ModelError::InvalidInput);
    }

    println!(
        "[RelationshipViewer] get_relationship_detail for model: {}, cubsobject id: {}",
        model_id, id
    );

    let start_time = Instant::now();

    // Get app state
    let app_state: Data<app_state::AppState> = extract()
        .await
        .map_err(|_| ModelError::ModelNotFound(model_id.clone()))?;
    log!("App state -> {:?}", app_state);

    // Get model
    let model = app_state
        .get_model_cache()
        .get(&model_id, &version)
        .ok_or(ModelError::ModelNotFound(model_id.clone()))?;
    let elements = &model.elements;
    let relationship = &model.relationships;

    // Get graph
    let graph_cache = app_state.get_graph_cache();
    let graph = match graph_cache.get(&model_id, &version) {
        Some(graph) => {
            println!("[RelationshipViewer - get_relationship_detail: Found graph in cache]");
            graph
        }
        None => {
            println!(
                "[RelationshipViewer - get_relationship_detail: Graph not found cache. Building..]"
            );
            // Build graph if not found
            let built_graph = ElementConnectorBuilder::build_graph(elements, relationship)?;

            // Add to cache
            graph_cache.insert(&model_id, &version, &built_graph);

            built_graph
        }
    };

    // From graph parse relationship

    // Construct into output

    Ok("LOADED".to_string())
}
