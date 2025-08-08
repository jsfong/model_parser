use leptos::{logging::log, prelude::*, reactive::spawn_local};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

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
    // let count = RwSignal::new(0);
    // let on_click = move |_| *count.write() += 1;

    // let  = signal("Model id in UUID".to_string());
    // let on_click_connect_to_db = move |_| {
    //     spawn_local(async {
    //         parse_model(model_id.get().clone()).await;
    //     });
    // };

    let parse_model_action = ServerAction::<ParseModel>::new();
    let value = parse_model_action.value();

    let (result, set_result) = signal("".to_string());
  
    Effect::new(move |_| {
        if let Some(Ok(value)) = value.get() {
            set_result.set(value);
        }
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <br/>
        // <input type="text" bind:value=(model_id, set_model_id)/>

        // <button on:click=on_click_connect_to_db>"Connect"</button>
        <ActionForm action=parse_model_action>
          <input type="text" name="model_id" placeholder="Model Id"/>
          <button type="submit">Create Post</button>
        </ActionForm>

        <div>
            "Result: " {result}
        </div>
        // <p>"Resuk: " {model_id}</p>
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
pub async fn parse_model(model_id: String) -> Result<String, ServerFnError> {
    log!("Parsing model with id {}", model_id);
    use crate::model::database_util;
    let db_pool = database_util::connect_to_db().await;

    Ok("String".to_string())
}
