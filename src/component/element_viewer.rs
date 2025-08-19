use leptos::prelude::*;

#[component]
pub fn ElementViewer(model_id: ReadSignal<String>) -> impl IntoView {
    view! {
        <div>
            <label for="query">Json Path Query: </label>
            <input type="hidden" name="model_id" prop:value=model_id size=40 />
            <input type="text" name="query" size=40 value="$.*"/>
            <label for="depth">Depth: </label>
            <input type="number" id="depth" name="depth" min="0" max="100" step="1" value="3" />
            <label for="limit">Limit: </label>
            <input type="number" id="limit" name="limit" min="0" max="5000" step="1" value="100" />
            <button type="submit">Run Query</button>
        </div>
    }
}
