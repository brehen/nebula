use std::path::Path;

use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use nebula_lib::list_files::list_files;
use nebula_server::utilities::run_wasm_module::run_wasm_module;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    let api_router = Router::new().route("/wasm/:module/:input", get(run_wasm_module));
    //.route("/docker/:module/:input", post(todo!()))
    // .with_state(app_state);

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(home))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );
    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    modules: Vec<String>,
}

async fn home() -> impl IntoResponse {
    let modules = list_files("/Users/mariuskluften/projects/wasm_modules")
        .expect("There to be modules on the server");
    info!("{:?}", modules);
    let modules: Vec<String> = modules
        .iter()
        .filter_map(|path| Path::new(path).file_stem())
        .map(|name| name.to_str().unwrap().to_string())
        .collect();
    info!("{:?}", modules);
    let template = HomeTemplate { modules };
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

// #[derive(Template)]
// #[template(path = "todo-list.html")]
// struct TodoList {
//     todos: Vec<String>,
// }

// #[derive(Deserialize)]
// struct TodoRequest {
//     todo: String,
// }

// async fn add_todo(
//     State(state): State<Arc<AppState>>,
//     Form(todo): Form<TodoRequest>,
// ) -> impl IntoResponse {
//     let mut lock = state.todos.lock().unwrap();
//     lock.push(todo.todo);
//
//     let template = TodoList {
//         todos: lock.clone(),
//     };
//
//     HtmlTemplate(template)
// }
