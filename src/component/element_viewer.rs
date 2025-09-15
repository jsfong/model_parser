use leptos::prelude::*;

use crate::app::RHSMode;

#[component]
pub fn ElementViewerInput(
    model_id: ReadSignal<String>,
    version: ReadSignal<String>,
    types: ReadSignal<Vec<String>>,
    natures: ReadSignal<Vec<String>>,
    set_query: WriteSignal<String>,
    set_rhs_mode: WriteSignal<RHSMode>,
) -> impl IntoView {
    let (query_value, set_query_value) = signal("".to_string());
    // let clear_query = move |_| set_query_value.set(String::new());
    let clear_query_result = move |_| {
        set_query.set("Querying...".to_string());
        set_rhs_mode.set(RHSMode::ModelStats);
    };

    view! {
        <div class="flex-container-view-input">
            <h4 class="flex-container-view-input-heading">Element Filtering</h4>
            <div class="flex-container-view-input-row">
                <input type="hidden" name="model_id" prop:value=model_id size=40 />
                <input type="hidden" name="vers_no" prop:value=version />

                // Conditional filter
                <label for="id">Id :</label>
                <input type="text" name="id" size=40 value="" />

                <label for="types">Select Type :</label>
                <select id="types" name="types">
                    {move || {
                        types
                            .get()
                            .into_iter()
                            .map(|n| {
                                let value = n.clone();
                                view! { <option value=n>{value}</option> }
                            })
                            .collect_view()
                    }}
                </select>

                <label for="natures">Nature :</label>
                <select id="natures" name="natures">
                    {move || {
                        natures
                            .get()
                            .into_iter()
                            .map(|n| {
                                let value = n.clone();
                                view! { <option value=n>{value}</option> }
                            })
                            .collect_view()
                    }}
                </select>
            </div>

            <h4 class="flex-container-view-input-heading">Facet Filtering</h4>
            <div class="flex-container-view-input-row">
                <label for="natures">Facet Type :</label>
                <select id="facet_type" name="facet_type">
                    <option value="">None</option>
                    <option value="dynamicFacets">Dynamic Facets</option>
                    <option value="coreFacets">Core Facets</option>
                    <option value="facets">Facets</option>
                </select>
                <label for="query">JSON Pointer:</label>
                <input
                    type="text"
                    name="query"
                    size=50
                    prop:value=query_value
                    on:input=move |ev| {
                        set_query_value.set(event_target_value(&ev));
                    }
                />

                <label for="query">Detail:</label>
                <input type="checkbox" name="is_detail" value="is_detail" />

            </div>

            <h4 class="flex-container-view-input-heading">Output Filtering</h4>
            <div class="flex-container-view-input-row">
                <label for="depth">Depth:</label>
                <input type="number" id="depth" name="depth" min="0" max="100" step="1" value="3" />
                <label for="limit">Limit:</label>
                <input
                    type="number"
                    id="limit"
                    name="limit"
                    min="0"
                    max="5000"
                    step="1"
                    value="100"
                />
                <button type="submit" on:click=clear_query_result>
                    Run Query
                </button>
                <input type="reset" value="Clear" />
            </div>
        </div>
    }
}
