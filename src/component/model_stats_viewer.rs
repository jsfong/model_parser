use crate::model::model_dict::{ElementCounts, ModelStats};
use leptos::{logging::log, prelude::*};
use serde_json::Value;

#[component]
pub fn ModelStatsViewer(model_stats: Memo<Option<Value>>) -> impl IntoView {
    log!("[ModelStatsViewer]");

    view! {
        <div class="flex-container-rhs-model-stats">
            <h2 class="label-model-stats">Model Stats</h2>
            {move || {
                //Parse stats
                match model_stats.get() {
                    Some(value) => {
                        match serde_json::from_value::<ModelStats>(value) {
                            Ok(model_stats) => view! {<ModelStats model_stats=&model_stats />}.into_any(),
                            Err(_) => view! {<ModelStatsNotFound />}.into_any(),
                        }
                    }
                    None => ModelStatsNotFound().into_any(),
                }
            }}
        </div>
    }
}

#[component]
fn ModelStats<'a>(model_stats: &'a ModelStats) -> impl IntoView {

    let cubs_obj_count = model_stats.elements_stats.as_ref().map_or(0, |r| r.all_count);
    let rel_count = model_stats.relationships_stats.as_ref().map_or(0, |r| r.all_count);
    let default_element_counts = ElementCounts::default();
    let cubs_obj_by_type: &ElementCounts = model_stats
        .elements_stats
        .as_ref()
        .map_or(&default_element_counts, |r| &r.by_type);

    let cubs_obj_by_nature: &ElementCounts = model_stats
        .elements_stats
        .as_ref()
        .map_or(&default_element_counts, |r| &r.by_nature);

    let rel_by_type: &ElementCounts = model_stats
        .relationships_stats
        .as_ref()
        .map_or(&default_element_counts, |r| &r.by_type);

    let rel_by_nature: &ElementCounts = model_stats
        .relationships_stats
        .as_ref()
        .map_or(&default_element_counts, |r| &r.by_nature);

    view! {
        <h3 class="label-model-stats">"Cubs Objects: " {cubs_obj_count}</h3>
        <h3 class="label-model-stats">"Version: " {model_stats.version}</h3>
        <h4 class="label-model-stats">By type</h4>
        <div class="table-model-stats">         
            <table>
                {
                    cubs_obj_by_type.value.iter().map(|c| {
                        view! { <tr><td>{c.element.clone()}</td> <td>{c.count}</td></tr>}
                    }).collect_view()
                }
            </table>
        </div>

        <h4 class="label-model-stats">By nature</h4>
        <div class="table-model-stats">         
            <table>
                {
                    cubs_obj_by_nature.value.iter().map(|c| {
                        view! { <tr><td>{c.element.clone()}</td> <td>{c.count}</td></tr>}
                    }).collect_view()
                }
            </table>
        </div>

        <br />
        <br />
        <h3 class="label-model-stats">"Relationships: " {rel_count}</h3>
        <div class="table-model-stats">         
            <table>
                {
                    rel_by_type.value.iter().map(|c| {
                        view! { <tr><td>{c.element.clone()}</td> <td>{c.count}</td></tr>}
                    }).collect_view()
                }
            </table>
        </div>

        <h4 class="label-model-stats">By nature</h4>
        <div class="table-model-stats">         
            <table>
                {
                    rel_by_nature.value.iter().map(|c| {
                        view! { <tr><td>{c.element.clone()}</td> <td>{c.count}</td></tr>}
                    }).collect_view()
                }
            </table>
        </div>
    }
}

#[component]
fn ModelStatsNotFound() -> impl IntoView {
    view! {
       <span class="json-error">"No Model Stats"</span>
    }
}
