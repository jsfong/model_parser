use leptos::{logging::log, prelude::*};
use serde_json::Value;

#[component]
pub fn JsonValue  (value:Value, depth: usize) -> impl IntoView {
    let indent_style = format!("margin-left: {}px", depth * 1);

    match value {
        Value::Null => view! {
            <div class="json-line" style=indent_style>
                <span class="json-null">"null"</span>
            </div>
        }
        .into_any(),

        Value::Bool(b) => view! {
            <div class="json-line" style=indent_style>
                <span class="json-boolean">{b.to_string()}</span>
            </div>
        }
        .into_any(),

        Value::Number(n) => view! {
            <div class="json-line" style=indent_style>
                <span class="json-number">{n.to_string()}</span>
            </div>
        }
        .into_any(),

        Value::String(s) => view! {
            <div class="json-line" style=indent_style>
                <span class="json-string">"\""</span>
                <span class="json-string">{s}</span>
                <span class="json-string">"\""</span>
            </div>
        }
        .into_any(),

        Value::Array(arr) => {
            let (is_collapsed, set_is_collapsed) = create_signal(false);

            view! {
                <div class="json-array">
                    <div class="json-line" style=indent_style.clone()>
                        <span
                            class="json-toggle"
                            class:collapsed=is_collapsed
                            on:click=move |_| set_is_collapsed.update(|c| *c = !*c)
                        >
                            "▼"
                        </span>
                        <span class="json-punctuation">"["</span>
                        {if is_collapsed.get() { //No need move
                            format!(" ... {} items ]", arr.len())
                        } else {
                            String::new()
                        }}
                    </div>

                    <div
                        class="json-content"
                        class:collapsed=is_collapsed
                    >
                    {   
                        let arr_len = arr.len().clone();
                        arr.into_iter().enumerate().map(|(i, item)| {
                            let is_last = i == arr_len - 1;
                            view! {
                                <div class="json-array-item">
                                    <JsonValue value=item depth=depth + 1 />
                                    {if !is_last {
                                        view! {
                                            <span class="json-punctuation">","</span>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                </div>
                            }
                        }).collect_view()}
                    </div>

                    {move || if !is_collapsed.get() {
                        view! {
                            <div class="json-line" style=indent_style.clone()>
                                <span class="json-punctuation">"]"</span>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>
            }
            .into_any()
        }

        Value::Object(obj) => {
            let (is_collapsed, set_is_collapsed) = create_signal(false);
            let keys: Vec<(String, Value)> = obj.into_iter().collect();

            view! {
                <div class="json-object">
                    <div class="json-line" style=indent_style.clone()>
                        <span
                            class="json-toggle"
                            class:collapsed=is_collapsed
                            on:click=move |_| set_is_collapsed.update(|c| *c = !*c)
                        >
                            "▼"
                        </span>
                        <br />

                       
                        <span class="json-punctuation">"{"</span>

                     
                        <div class="json-keys-number">
                        {
                            let key_len = keys.len().clone();
                            move || if is_collapsed.get() {
                            format!(" ... {} keys ", key_len)
                        } else {
                            String::new()
                        }}
                        </div>
                        
                    </div>

                    <div
                        class="json-content"
                        class:collapsed=is_collapsed
                    >
                        {   let key_len = keys.len().clone();
                            keys.into_iter().enumerate().map(|(i, (key, value))| {
                            let is_last = i == key_len - 1;
                            let mut is_value = false;
                            let key_indent = format!("margin-left: {}px", (depth + 1) * 5);

                            view! {
                                <div class="json-object-item">
                                    <div class="json-line" style=key_indent>
                                        <span class="json-key">"\""</span>
                                        <span class="json-key">{key}</span>
                                        <span class="json-key">"\""</span>
                                        <span class="json-punctuation">": "</span>

                                        {match value {
                                            Value::Object(_) | Value::Array(_) => {
                                                is_value = false;
                                                view! {
                                                    <div style="display: inline">
                                                        <JsonValue value=value depth=depth + 1 />
                                                    </div>
                                                }.into_any()
                                            },
                                            _ => {
                                                is_value = true;
                                                view! {
                                                    <div style="display: inline">
                                                        <JsonValueInline value=value />
                                                    </div>
                                                }.into_any()
                                            }
                                        }}

                                        {if !is_last && is_value{
                                            view! {
                                                <span class="json-punctuation">","</span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>

                    {move || if !is_collapsed.get() {
                        view! {
                            <div class="json-line" style=indent_style.clone()>
                                <span class="json-punctuation">"}"</span>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>
            }
            .into_any()
        }
    }
}

#[component]
pub fn JsonValueInline(value: Value) -> impl IntoView {
    match value {
        Value::Null => view! {
            <span class="json-null">"null"</span>
        }
        .into_any(),

        Value::Bool(b) => view! {
            <span class="json-boolean">{b.to_string()}</span>
        }
        .into_any(),

        Value::Number(n) => view! {
            <span class="json-number">{n.to_string()}</span>
        }
        .into_any(),

        Value::String(s) => view! {
            <span class="json-string">"\""</span>
            <span class="json-string">{s}</span>
            <span class="json-string">"\""</span>
        }
        .into_any(),

        _ => view! {
            <span class="json-error">"Complex value"</span>
        }
        .into_any(),
    }
}

#[component]
pub fn JsonNotFound() -> impl IntoView {
    view! {
       <span class="json-error">"Invalid JSON format"</span>
    }
}
