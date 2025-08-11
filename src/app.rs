use std::time::Instant;

use leptos::{logging::log, prelude::*};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerResult {
    pub value: String,
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
        <Title text="Welcome to Leptos"/>

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
    let (result, set_result) = signal("".to_string());
    let (duration, set_duration) = signal("".to_string());
    
    Effect::new(move |_| {
        if let Some(Ok(result)) = value.get() {
            set_result.set(result.value);
            set_duration.set(result.duration);
        }
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <br/>
        // <input type="text" bind:value=(model_id, set_model_id)/>

        // <button on:click=on_click_connect_to_db>"Connect"</button>
        <ActionForm action=parse_model_action>
          <input type="text" name="model_id" placeholder="Model Id" value="aa5bc4b2-156f-4bad-b13a-4ccf31df53ca"/>
          <button type="submit">Read model</button>
        </ActionForm>

        <div>
            <p>"Result: " </p>
            <div class="json-container light-theme"> {result} </div>
            <div> "Duration: " {duration}</div>
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
    log!("Parsing model with id {}", model_id);
    use crate::model::database_util;
    use crate::model::model_dict;
    use crate::model::parser;
    let start_time = Instant::now();

    //Connect to DB
    let db_pool = database_util::connect_to_db().await;

    // Read saved model
    let model_data = parser::read_model_data_from_db(&model_id, &db_pool).await;
    let model_data = match model_data {
        Ok(model_data) => model_data,
        Err(e) => {
            eprintln!("Unable to read saved Model");
            return Err(ServerFnError::ServerError(
                "Unable to read saved Model".to_string(),
            ));
        }
    };

    //Build stats
    let dict = model_dict::ModelDictionary::from(&model_data);
    let elapsed_time = start_time.elapsed();

    //Convert json string
    match serde_json::to_string_pretty(&dict.model_stats) {
        Ok(result) => Ok(ServerResult {
            value: result,
            duration: elapsed_time.as_millis().to_string() + " ms",
        }),
        Err(e) => {
            return Err(ServerFnError::ServerError(
                "Unable to convert dict to json string".to_string(),
            ))
        }
    }
}
