use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

use anyhow::Context;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use nebula_lib::{
    docker_runner::run_docker_image, list_files::list_files, models::FunctionResult,
    wasm_runner::run_wasi_module,
};
use nebula_server::utilities::{get_file_path::get_file_path, run_wasm_module::run_wasm_module};
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    function_calls: Mutex<Vec<FunctionResult>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nebula_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let api_router = Router::new()
        .route("/wasm/:module/:input", get(run_wasm_module))
        .route("/wasm", post(call_function));
    //.route("/docker/:module/:input", post(todo!()))
    // .with_state(app_state);
    //
    let app_state = Arc::new(AppState {
        function_calls: Mutex::new(vec![]),
    });

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(home))
        .route("/wasm", get(wasm))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(app_state)
        .layer(LiveReloadLayer::new());
    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Template)]
#[template(path = "pages/wasm.rs.html")]
struct WasmTemplate {
    modules: Vec<String>,
}

async fn wasm() -> impl IntoResponse {
    let modules = list_files("/Users/mariuskluften/projects/modules/wasm")
        .expect("There to be modules on the server");
    let modules: Vec<String> = modules
        .iter()
        .filter_map(|path| Path::new(path).file_stem())
        .map(|name| name.to_str().unwrap().to_string())
        .collect();
    let template = WasmTemplate { modules };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/home.rs.html")]
struct HomeTemplate {}

async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),

            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template, Debug)]
#[template(path = "components/function_results.rs.html")]
struct FCList {
    function_results: Vec<FunctionResult>,
}

#[derive(Deserialize)]
enum ModuleType {
    Docker,
    Wasm,
}

#[derive(Deserialize)]
struct FunctionRequest {
    function_name: String,
    input: String,
    module_type: ModuleType,
}

async fn call_function(
    State(state): State<Arc<AppState>>,
    Form(request): Form<FunctionRequest>,
) -> impl IntoResponse {
    info!("calling function: {:?}", request.function_name);
    let result: FunctionResult = match request.module_type {
        ModuleType::Docker => {
            let docker_module = format!("brehen/{}", request.input);
            run_docker_image(&docker_module, &request.input).expect("It to work")
        }
        ModuleType::Wasm => {
            let function_path = get_file_path(&request.function_name);
            run_wasi_module(&function_path, &request.input).expect("to work")
        }
    };
    let mut lock = state.function_calls.lock().await;
    lock.push(result);

    let template = FCList {
        function_results: lock.clone().into_iter().rev().collect(),
    };

    HtmlTemplate(template)
}
