use leptos::prelude::*;

#[component]
pub fn ElementViewerInput(
    model_id: ReadSignal<String>,
    version: ReadSignal<String>,
    types: ReadSignal<Vec<String>>,
    natures: ReadSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <div class="flex-container-view-input">
            <div class="flex-container-view-input-row">
                <input type="hidden" name="model_id" prop:value=model_id size=40 />
                <input type="hidden" name="vers_no" prop:value=version />

                // Conditional filter
                <label for="id">Id : </label>
                <input type="text" name="id" size=40 value=""/>

                <label for="types">Select Type : </label>
                <select id="types" name="types">
                    { move || 
                        types.get().into_iter().map(|n| {
                            let value = n.clone();
                            view! {
                                <option value={n}>{value}</option>
                            }
                        }).collect_view()
                    }
                </select>

                <label for="natures">Nature : </label>
                <select id="natures" name="natures">
                    { move ||
                        natures.get().into_iter().map(|n| {
                            let value = n.clone();
                            view! {
                                <option value={n}>{value}</option>
                            }
                        }).collect_view()
                    }
                </select>
            </div>

            <div class="flex-container-view-input-row">
                <label for="query">Json Path Query: </label>
                <input type="text" name="query" size=100 value="$.*"/>
             </div>
             <div class="flex-container-view-input-row">
                <label for="depth">Depth: </label>
                <input type="number" id="depth" name="depth" min="0" max="100" step="1" value="3" />
                <label for="limit">Limit: </label>
                <input type="number" id="limit" name="limit" min="0" max="5000" step="1" value="100" />
                <button type="submit">Run Query</button>
            </div>
        </div>
    }
}
