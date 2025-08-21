use crate::{
    component::{
        element_viewer::ElementViewerInput,
        json_viewer::{self},
        model_stats_viewer,
    },
    model::cubs_model::{self, ModelVersionNumber},
};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerResult {
    pub model_id: String,
    pub stats: String,
    pub types: Vec<String>,
    pub natures: Vec<String>,
    pub duration: String,
    pub model_versions: Vec<ModelVersionNumber>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct QueryResult {
    pub data: String,
    pub duration: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_model_parser.css"/>

        // sets the document title
        <Title text="Model Parser"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let parse_model_action = ServerAction::<ParseModel>::new();
    let value = parse_model_action.value();

    let query_model_action = ServerAction::<QueryModel>::new();
    let query_value = query_model_action.value();

    // Signal
    let (model_id, set_model_id) = signal("".to_string());
    let (model_versions, set_model_versions): (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>) =
        signal(vec!["".to_string()]);
    let (selected_version, set_selected_version) = signal("".to_string());

    let (stats, set_stats) = signal("".to_string());
    let (result, set_result) = signal("".to_string());
    let (duration, set_duration) = signal("".to_string());
    let (query, set_query) = signal("".to_string());
    let (element_type, set_element_type): (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>) =
        signal(vec!["".to_string()]);
    let (element_nature, set_element_nature): (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>) =
        signal(vec!["".to_string()]);

    let parsed_json_stats = Memo::new(move |_| {
        let stats_str = stats.get();
        let parsed = serde_json::from_str::<Value>(&stats_str).ok();
        parsed
    });

    Effect::new(move |_| {
        if let Some(Ok(result)) = value.get() {
            set_model_id.set(result.model_id.clone());
            set_stats.set(result.stats.clone());

            // Version
            let versions: Vec<String> = result
                .model_versions
                .iter()
                .map(|mv| mv.vers_no.to_string())
                .collect();
            let latest_version = match versions.first(){
                Some(v) => v.clone(),
                None => 0.to_string(),
            };
            log!("Version: {:?}", versions);
            set_model_versions.set(versions);        
            set_selected_version.set(latest_version);

            set_duration.set(result.duration.clone());
            set_query.set(String::new());

            // Type and natures
            let mut types = result.types;
            let mut natures = result.natures;
            types.sort();
            natures.sort();
            types.insert(0, "All".to_string());
            natures.insert(0, "All".to_string());
            set_element_type.set(types);
            set_element_nature.set(natures);
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(result)) = query_value.get() {
            log!("[HomePage] Query result received: {:?}", result);
            set_query.set(result.data);
            set_duration.set(result.duration);
        }
    });
    let parsed_query = Memo::new(move |_| serde_json::from_str::<Value>(&query.get()).ok());

    view! {
        <h1>"Model Parser"</h1>
        <br/>

        // Input
        <ActionForm action=parse_model_action >
            <div class="flex-cmd-parent">
                <label for="model_id">Model Id: </label>
                <input type="text" name="model_id" placeholder="Model Id" size=40 value="aa5bc4b2-156f-4bad-b13a-4ccf31df53ca" class="flex-cmd-model-id"/>
                <label for="vers_no">Version No: </label>
                <select id="vers_no" name="vers_no" on:change= move |ev|{
                    let value = event_target_value(&ev);
                    set_selected_version.set(value);
                }>
                    // <option value="">Empty</option>
                    {move ||
                        model_versions.get().into_iter().map(|v|{
                            let value = v.clone();
                            view! {
                                <option value={value}>{v}</option>                                
                            }
                        }).collect_view()
                    }
                </select>
                <button type="submit" class="flex-cmd-item">Read model</button>
            </div>
        </ActionForm>


        <br />

        //View
        <div class="flex-container">
            <div class="flex-container-view">
            // Element viewer
            { move ||
                if !model_id.get().is_empty() {
                    view! {
                            <ActionForm action=query_model_action>
                                <ElementViewerInput model_id=model_id version=selected_version types=element_type natures=element_nature/>
                            </ActionForm>
                            <json_viewer::JsonViewer json_value=parsed_query collapsed=false/>
                            <div> "Duration: " {duration}</div>
                        }.into_any()
                    } else {
                        view!{}.into_any()
                    }
                }
            </div>


            // RHS
            <div class="flex-container-rhs">
                <model_stats_viewer::ModelStatsViewer model_stats=parsed_json_stats />
            </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server(ParseModel, "/api")]
pub async fn parse_model(model_id: String, vers_no: String) -> Result<ServerResult, ServerFnError> {
    log!("[parse_model] Parsing model with id {}", model_id);
    use crate::model::model_dict;
    use crate::model::parser;
    use leptos::logging::log;
    use std::time::Instant;
    let start_time = Instant::now();

    //Read all model version
    let model_versions = parser::read_model_data_versions(&model_id)
        .await
        .unwrap_or_default();

    // Read saved model
    let version_num = vers_no
        .parse::<i32>()
        .unwrap_or_else(|_| model_versions.first().map_or(0, |v| v.vers_no));
    let model_data = parser::read_model_data(&model_id, version_num).await;
    let model_data = match model_data {
        Ok(model_data) => model_data,
        Err(_) => {
            eprintln!("Unable to read saved Model");
            return Err(ServerFnError::ServerError(
                "Unable to read saved Model".to_string(),
            ));
        }
    };

    //Build stats
    let dict = model_dict::ModelDictionary::from(&model_data);

    //Convert json string
    let model_stats = serde_json::to_string_pretty(&dict.model_stats).unwrap();
    // let elements = serde_json::to_string_pretty(&model_data.elements_json_path("$[:1]")).unwrap();

    let elapsed_time = start_time.elapsed();

    log!(
        "[parse_model] Successfully parse model with id {} \n",
        model_id
    );

    Ok(ServerResult {
        model_id: model_id,
        stats: model_stats,
        types: dict.get_element_types(),
        natures: dict.get_element_nature(),
        duration: format!("Get model took {} ms", elapsed_time.as_millis().to_string()),
        model_versions: model_versions,
    })
}

#[server(QueryModel, "/api")]
pub async fn query_model(
    model_id: String,
    vers_no: String,
    id: String,
    types: String,
    natures: String,
    query: String,
    depth: usize,
    limit: usize,
) -> Result<QueryResult, ServerFnError> {
    use crate::model::parser;
    use leptos::logging::log;
    use std::time::Instant;
    log!(
        "[query_model] Parsing model with id {} with type {} and nature {} and version {}",
        model_id,
        types,
        natures,
        vers_no,
    );

    if model_id.is_empty() || query.is_empty() {
        return Ok(QueryResult::default());
    }

    log!(
        "[query_model] Querying model: {} with query: {} with depth: {} with limit: {}",
        model_id,
        query,
        depth,
        limit,
    );

    let start_time = Instant::now();

    // Read saved model
    let version_num = vers_no.parse::<i32>().unwrap_or_else(|_| 0);
    let model_data = parser::read_model_data(&model_id, version_num).await;
    let model_data = match model_data {
        Ok(model_data) => model_data,
        Err(_) => {
            eprintln!("Unable to read saved Model");
            return Err(ServerFnError::ServerError(
                "Unable to read saved Model".to_string(),
            ));
        }
    };

    //Filtering
    //filter id
    let mut filtered_elements = match id.is_empty() {
        true => model_data.get_elements(),
        false => match model_data.get_element_with_id(&id) {
            Some(e) => vec![e],
            None => vec![],
        },
    };

    //filter nature
    filtered_elements.retain(|e| match natures.as_str() {
        "All" => true,
        _ => *e.nature == natures,
    });

    //filter type
    filtered_elements.retain(|e| match types.as_str() {
        "All" => true,
        _ => *e.type_ == types,
    });

    //TODO jsonPath Query
    // let query_result = model_data.execute_json_path_for_element(&query);
    // let value = serde_json::to_value(model_data.elements).unwrap_or_default();

    //Limit
    let limit = match limit >= filtered_elements.len() {
        true => filtered_elements.len(),
        false => limit,
    };

    println!(
        "[query_model] limiting total result of {} to {}",
        filtered_elements.len(),
        limit
    );
    let limited_query_result = &filtered_elements[0..limit].to_vec();

    //Depth
    println!(
        "[query_model] truncating {} elements to depth {}",
        limited_query_result.len(),
        depth
    );

    //Conver to Vec<Value>
    let limited_query_result_value: Vec<Value> = limited_query_result
        .iter()
        .map(|e| serde_json::to_value(e).unwrap_or_default())
        .filter(|v| *v != Value::Null)
        .collect();

    let elements = match depth > 0 {
        true => {
            let filtered_element = cubs_model::truncate_value(&limited_query_result_value, depth);
            serde_json::to_string_pretty(&filtered_element).unwrap()
        }
        false => serde_json::to_string_pretty(&limited_query_result).unwrap(),
    };
    let elapsed_time = start_time.elapsed();

    log!(
        "[parse_model] Successfully query model with id {} \n",
        model_id
    );

    Ok(QueryResult {
        data: elements,
        duration: format!(
            "Query model took {} ms",
            elapsed_time.as_millis().to_string()
        ),
    })
}
