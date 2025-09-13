use leptos::{ html::P, logging::log, prelude::*};
use serde_json::Value;

const PADDING: i32 = 10;

#[component]
pub fn JsonViewer(json_value: Memo<Option<Value>>, collapsed: bool, set_selected_object_id: WriteSignal<Option<String>>) -> impl IntoView {
    view! {
         <div class="json-container">
            <div class="json-viewer">
                {move || {
                    if let Some(v) = json_value.get() {
                        view! {<JsonNode value=v level=1 is_last=true  collapsed=collapsed key=None set_selected_object_id=set_selected_object_id/>}.into_any()
                    }else{
                        view! {<JsonNotFound />}.into_any()
                    }
                }}
            </div>
        </div>
    }


}


#[component]
fn JsonNode(value: Value, level: i32, is_last: bool, collapsed: bool, key: Option<String>, set_selected_object_id: WriteSignal<Option<String>>) -> impl IntoView {
    // let indent_style = format!("margin-left: {}px;", level * PADDING);
    
    match value {
        Value::Object(obj) => {
            let (is_collapsed, set_collapsed) = signal(collapsed);
            let entries: Vec<(String, Value)> = obj.into_iter().collect();
            let obj_len = entries.len();
            
            if obj_len == 0 {
                view! {
                    <span>
                        <span class="json-brace">"{}"</span>
                        {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                    </span>
                }.into_any()
            } else {
                view! {
                    <div class="json-object">
                        <span 
                            class="json-toggle"
                            on:click=move |_| set_collapsed.update(|c| *c = !*c)
                        >
                            {move || if is_collapsed.get() { "▶" } else { "▼" }}
                        </span>
                        <span class="json-brace">"{"</span>
                        <div class="json-object-content" class:collapsed=is_collapsed>
                            {entries.into_iter().enumerate().map(|(i, (key, val))| {
                                let is_last_item = i == obj_len - 1;
                                let clone_key = Some(key.clone());
                                view! {
                                    <div class="json-property" style=format!("margin-left: {}px;", (level ) * PADDING)>
                                        <span class="json-key">"\""</span>
                                        <span class="json-key-text">{key}</span>
                                        <span class="json-key">"\""</span>
                                        <span class="json-colon">": "</span>
                                        <JsonNode value=val level=level + 1 is_last=is_last_item collapsed=collapsed key=clone_key set_selected_object_id=set_selected_object_id/>
                                    </div>
                                }
                            }).collect_view()}
                            <div style=format!("margin-left: {}px;", PADDING + 5)>
                                <span class="json-brace">"}"</span>
                                {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                            </div>
                        </div>
                    </div>
                }.into_any()
            }
        },
        Value::Array(arr) => {
            let (is_collapsed, set_collapsed) = signal(false);
            let length = arr.len();
            
            if length == 0 {
                view! {
                    <span>
                        <span class="json-brace">"[]"</span>
                        {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                    </span>
                }.into_any()
            } else {
                view! {
                    <div class="json-array">
                        <span 
                            class="json-toggle"
                            on:click=move |_| set_collapsed.update(|c| *c = !*c)
                        >
                            {move || if is_collapsed.get() { "▶" } else { "▼" }}
                        </span>
                        <span class="json-brace">"["</span>
                        <div class="json-array-content" class:collapsed=is_collapsed>
                            {arr.into_iter().enumerate().map(|(i, val)| {
                                let is_last_item = i == length - 1;
                                view! {
                                    <div class="json-array-item" style=format!("margin-left: {}px;", (level + 1) * PADDING)>
                                        <JsonNode value=val level=level + 1 is_last=is_last_item collapsed=collapsed key=None set_selected_object_id=set_selected_object_id/>
                                    </div>
                                }
                            }).collect_view()}
                            <div style=format!("margin-left: {}px;", level * 20)>
                                <span class="json-brace">"]"</span>
                                {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                            </div>
                        </div>
                    </div>
                }.into_any()
            }
        },
        Value::String(s) => {
            view! {
                <span class="json-string">
                    <span class="json-quote">"\""</span>
                    // <span class="json-string-content">{s}</span>
                    {
                       if let Some(field_key) = key  {

                        if field_key == "id" {
                            Some (
                                view!{
                                    <span class="json-string-content, tooltip">{s.clone()}
                                        <span class="tooltiptext" on:click=move |_| { set_selected_object_id.update(|c| *c=Some(s.clone()));}>Tooltip text</span> 
                                    </span>            
                                }.into_any()
                            )
                        } else {
                            Some(view!{<span class="json-string-content">{s}</span>}.into_any())
                        }
                        
                       } else {
                        Some(view!{<span class="json-string-content">{s}</span>}.into_any())
                       }
                    }
                    <span class="json-quote">"\""</span>
                    {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                </span>
            }.into_any()
        },
        Value::Number(n) => {
            view! {
                <span class="json-number">
                    {n.to_string()}
                    {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                </span>
            }.into_any()
        },
        Value::Bool(b) => {
            view! {
                <span class="json-boolean">
                    {b.to_string()}
                    {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                </span>
            }.into_any()
        },
        Value::Null => {
            view! {
                <span class="json-null">
                    "null"
                    {if !is_last { Some(view! { <span class="json-comma">","</span> }) } else { None }}
                </span>
            }.into_any()
        }
    }
}

#[component]
pub fn JsonNotFound() -> impl IntoView {
    view! {
       <span class="json-error">"No data or querying in progress."</span>
    }
}
