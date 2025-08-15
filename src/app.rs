use crate::component::{model_stats_viewer, json_viewer::{self}};
use leptos::prelude::*;
use leptos::logging::log;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerResult {
    pub stats: String,
    pub elements: String,
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
    let (stats, set_stats) = signal("".to_string());
    let (result, set_result) = signal("".to_string());
    let (duration, set_duration) = signal("".to_string());

    let parsed_json_stats = Memo::new(move |_| {
        let stats_str = stats.get();
        log!("[HomePage] Stats string: {:?}", stats_str);
        let parsed = serde_json::from_str::<Value>(&stats_str).ok();
        log!("[HomePage] Parsed stats: {:?}", parsed);
        parsed
    });
    let parsed_json_elements =
        Memo::new(move |_| serde_json::from_str::<Value>(&result.get()).ok());

    Effect::new(move |_| {
        if let Some(Ok(result)) = value.get() {
            log!("[HomePage] Server result received: {:?}", result);
            set_stats.set(result.stats.clone());
            set_result.set(result.elements.clone());
            set_duration.set(result.duration.clone());
            log!("[HomePage] Set stats to: {:?}", result.stats);
        }
    });

    view! {
        <h1>"Model Parser"</h1>
        <br/>

        // Input
        <ActionForm action=parse_model_action >
            <div class="flex-cmd-parent">
                <input type="text" name="model_id" placeholder="Model Id" size=40 value="aa5bc4b2-156f-4bad-b13a-4ccf31df53ca" class="flex-cmd-model-id"/>
                <label for="vers">Version No: </label>
                <select id="vers" name="vers_no">
                    <option value="">Empty</option>
                </select>
                <button type="submit" class="flex-cmd-item">Read model</button>
            </div>
        </ActionForm>


        <br />

        //View
        <div class="flex-container">

            // Json viewerr
            <div class="flex-container-view">
                <json_viewer::JsonViewer json_value=parsed_json_stats collapsed=false/>
                <json_viewer::JsonViewer json_value=parsed_json_elements collapsed=true/>

                <div> "Duration: " {duration}</div>
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
pub async fn parse_model(model_id: String) -> Result<ServerResult, ServerFnError> {
    log!("[parse_model] Parsing model with id {}", model_id);
    use crate::model::database_util;
    use crate::model::model_dict;
    use crate::model::parser;
    use leptos::logging::log;
    use std::time::Instant;
    let start_time = Instant::now();

    //Connect to DB
    let db_pool = database_util::connect_to_db().await;

    // Read saved model
    let model_data = parser::read_model_data(&model_id, &db_pool).await;
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
    let elements = serde_json::to_string_pretty(&model_data.elements_json_path("$[:1]")).unwrap();

    let elapsed_time = start_time.elapsed();

    log!("[parse_model] Parsing model with id {} \n", model_id);

    Ok(ServerResult {
        stats: model_stats,
        elements,
        duration: elapsed_time.as_millis().to_string() + " ms",
    })
}
