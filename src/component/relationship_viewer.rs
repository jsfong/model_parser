use std::collections::HashMap;

use crate::model::model_error::ModelError;
use leptos::{logging::log, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OutputToken<T> {
    Tab,
    Value(T),
    InArrow,
    OutArrow,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OutputLine<T> {
    pub line: Vec<OutputToken<T>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OutputGraph<T> {
    pub parent_lines: Vec<OutputLine<T>>,
    pub child_lines: Vec<OutputLine<T>>,
    pub elements_data: HashMap<String, Value>,
}

pub enum RelationshipDirection {
    Parent,
    Child,
}

impl<T> OutputLine<T> {
    pub fn new() -> Self {
        Self { line: Vec::new() }
    }

    pub fn push(&mut self, token: OutputToken<T>) {
        self.line.push(token);
    }
}

#[component]
pub fn RelationshipViewer(
    model_id: ReadSignal<String>,
    selected_version: ReadSignal<String>,
    selected_object_id: ReadSignal<String>,
    set_selected_object_id: WriteSignal<String>,
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
                Ok(r) => view! { <RelationshipNodeRender data=r set_selected_object_id=set_selected_object_id /> }.into_any(),
                Err(e) => view! { <span>"error "</span> }.into_any(),
            },
            None => view! { <span>"Loading ... "</span> }.into_any(),
        }
    };

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading initial data..."</p> }
        }>
            <h2 class="label-model-stats">Relationship Stats</h2>
            {move || async_relationship_detail_result}
        </Transition>
    }

    // view! { <span>"Relationship viewer for "{async_relationship_detail_result}</span> }
}

#[component]
pub fn RelationshipNodeRender(data: OutputGraph<String>, set_selected_object_id: WriteSignal<String>) -> impl IntoView {
    let elements_store = data.elements_data;
    let parent_lines = data.parent_lines;
    let child_lines = data.child_lines;
    view! {
        <h3 class="label-model-stats">"Parents: "</h3>
        {parent_lines
            .iter()
            .map(|output_line| {
                view! {
                    <div class="relationship-viewer-flex-parent">
                        {output_line
                            .line
                            .iter()
                            .map(|token| {
                                view! {
                                    <OutputTokenRender
                                        token=token.clone()
                                        elements_store=&elements_store
                                        set_selected_object_id=set_selected_object_id
                                    />
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                }
            })
            .collect::<Vec<_>>()}

        <h3 class="label-model-stats">"Childs: "</h3>
        {child_lines
            .iter()
            .map(|output_line| {
                view! {
                    <div class="relationship-viewer-flex-parent">
                        {output_line
                            .line
                            .iter()
                            .map(|token| {
                                view! {
                                    <OutputTokenRender
                                        token=token.clone()
                                        elements_store=&elements_store
                                        set_selected_object_id=set_selected_object_id
                                    />
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                }
            })
            .collect::<Vec<_>>()}
    }
}

#[component]
pub fn  OutputTokenRender<'a>(token: OutputToken<String>, elements_store: &'a HashMap<String, Value>, set_selected_object_id: WriteSignal<String>) -> impl IntoView {

    match token {
        OutputToken::Tab => view! { <span class="relationship-viewer-flex-line, relationship-viewer-token-tab"></span> }.into_any(),
        OutputToken::Value(v) => {
            let element = match elements_store.get(&v) {
                Some(element) => element.clone(),
                None => Value::Null,
            };
            
            view! {
                <span class="relationship-viewer-flex-line, relationship-viewer-flex-value-tooltip">
                    // {v}
                    <RenderJsonValue value=element.clone() key="name".to_string() />
                    :
                    <RenderJsonValue value=element.clone() key="type".to_string() />
                    <div class="relationship-viewer-flex-value-tooltiptext">
                        <OutputTooltipRender
                            value=element
                            set_selected_object_id=set_selected_object_id
                        />
                    </div>
                </span>
            }.into_any()    
        },

        OutputToken::InArrow => view! { <span class="relationship-viewer-flex-line">"➚"</span> }.into_any(),
        OutputToken::OutArrow => view! { <span class="relationship-viewer-flex-line">"➘"</span> }.into_any(),
    }
}

#[component]
pub fn  OutputTooltipRender(value: Value, set_selected_object_id: WriteSignal<String>) -> impl IntoView {

    let id: Option<String> =  value.get("id").map(|id| id.to_string());

     let mut rows = Vec::new();
     rows.push(view! {
         <RenderJsonValueToTD
             value=value.clone()
             key="id".to_string()
             label="Id".to_string()
         />
     }.into_any());
     rows.push(view! {
         <RenderJsonValueToTD value=value.clone() key="type".to_string() label="Type".to_string() />
     }.into_any());
     rows.push(view! {
         <RenderJsonValueToTD value=value.clone() key="name".to_string() label="Name".to_string() />
     }.into_any());
     rows.push(view! {
         <RenderJsonValueToTD
             value=value.clone()
             key="nature".to_string()
             label="Nature".to_string()
         />
     }.into_any());
    
    view! { 
        <table>{rows}</table>
        <button  //  Search of new relationship when click the id
             on:click=move |_| {
                 match &id {
                     Some(id) => set_selected_object_id.set(id.to_string().replace("\"", "")),
                     None => {}
                 }
             }>Check Relationship</button>
     }


}

#[component]
fn RenderJsonValueToTD (value: Value, key: String, label: String) -> impl IntoView {

    if let Some(value) = value.get(key) {
        view! {
            <tr>
                <td>{label.clone()}": "</td>
                <td>{value.to_string()}</td>
            </tr>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}

#[component]
fn RenderJsonValue (value: Value, key: String) -> impl IntoView {

    if let Some(value) = value.get(key) {
        view! {
            {match value == "" {
                true => "\"\"".to_string(),
                false => value.to_string().replace("\"", ""),
            }}
        }.into_any()
    } else {
        view! {}.into_any()
    }
}

#[server(GetRelationship, "/api")]
pub async fn get_relationship_detail(
    model_id: String,
    version: String,
    id: String,
) -> Result<OutputGraph<String>, ModelError> {
    use crate::model::{
        app_state,
        element_graph::{ElementConnector, ElementGraph},
        element_graph_parser::ElementGraphParser,
        element_parser::ElementConnectorBuilder,
        model_error::ModelError,
    };
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
            println!("[RelationshipViewer - get_relationship_detail]: Found graph in cache");
            graph
        }
        None => {
            println!(
                "[RelationshipViewer - get_relationship_detail]: Graph not found cache. Building.."
            );
            // Build graph if not found
            let built_graph = ElementConnectorBuilder::build_graph(elements, relationship)?;

            // Add to cache
            graph_cache.insert(&model_id, &version, &built_graph);

            built_graph
        }
    };

    // From graph parse relationship and return part of the graph n parent and n layer of child
    let parse_graph = ElementGraphParser::parse_graph(&graph, &id, 1, 2)?;

    // Construct into output
    let output_graph = ElementGraphParser::build_output(&parse_graph, &id, &model);

    //Log time
    let elapsed_time = start_time.elapsed();
    println!(
        "[Execution time] {} - {:?}",
        "get_relationship_detail", elapsed_time
    );

    output_graph
}
