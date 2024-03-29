use clap::{arg, Parser};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::{net::TcpListener, sync::Mutex};
use tower_livereload::LiveReloadLayer;
// use tower_livereload::LiveReloadLayer;

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use nebula_server::{
    api::call_function::call_function,
    components::function_results::get_function_results,
    models::AppState,
    pages::{about, docker_page, index, metrics, wasm_page},
    utilities::{
        persist::load_results,
        serialize_modules::{self, serialize_modules},
    },
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = ServerArgs::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nebula_server=debug,tower_livereload=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Leave out https redirect for now
    //
    // let ports = Ports {
    //     http: options.port,
    //     https: 443,
    // };
    // if cfg!(not(debug_assertions)) && options.host.to_string() == "0.0.0.0" {
    //     info!("We in this production mode, running on 0.0.0.0");
    //     tokio::spawn(redirect_http_to_https(ports));
    // }

    info!("initializing router...");

    let api_router = Router::new()
        .route("/results", get(get_function_results))
        .route("/wasm", post(call_function))
        .route("/docker", post(call_function))
        .route("/wasm/all", post(call_function));

    let stored_function_calls = match load_results() {
        Ok(func_calls) => func_calls,
        Err(_) => vec![],
    };

    serialize_modules();

    let app_state = Arc::new(AppState {
        function_calls: Mutex::new(stored_function_calls),
    });

    let mut router = Router::new()
        .nest("/api", api_router)
        .route("/", get(index::home))
        .route("/about", get(about::about))
        .route("/metrics", get(metrics::metrics))
        .route("/wasm", get(wasm_page::wasm))
        .route("/docker", get(docker_page::docker))
        .nest_service("/assets", ServeDir::new(options.assets_path))
        .with_state(app_state);

    #[cfg(debug_assertions)]
    {
        info!("in that place");
        router = router.layer(LiveReloadLayer::new());
    }

    info!(
        "Up and running on address {}:{}!",
        options.host, options.port
    );

    let listener = TcpListener::bind(&SocketAddr::new(options.host, options.port))
        .await
        .unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ServerArgs {
    /// HTTP listening address.
    #[arg(short = 'e', long, default_value = "127.0.0.1")]
    pub host: IpAddr,
    /// HTTP listening port.
    #[arg(short = 'p', long, default_value = "8080")]
    pub port: u16,

    /// Asset location
    #[arg(short = 'a', long, default_value = "./assets")]
    pub assets_path: String,
}
