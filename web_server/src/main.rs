use clap::{arg, Parser};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::sync::Mutex;

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use nebula_server::{
    components::function_results::{call_function, AppState},
    pages::{docker_page, index, wasm_page},
    utilities::run_wasm_module::run_wasm_module,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = ServerOptions::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nebula_server=debug,tower_livereload=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    info!("Cwd path is: {:?}", assets_path);
    info!("Expected assets dir is: {:?}/assets", assets_path);
    let api_router = Router::new()
        .route("/wasm/:module/:input", get(run_wasm_module))
        .route("/wasm", post(call_function))
        .route("/docker", post(call_function));
    let app_state = Arc::new(AppState {
        function_calls: Mutex::new(vec![]),
    });

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(index::home))
        .route("/wasm", get(wasm_page::wasm))
        .route("/docker", get(docker_page::docker))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(app_state);
    //.layer(LiveReloadLayer::new());
    info!(
        "Up and running on address {}:{}!",
        options.address, options.port
    );

    axum::Server::bind(&SocketAddr::new(options.address, options.port))
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ServerOptions {
    /// HTTP listening address.
    #[arg(short = 'a', long, default_value = "127.0.0.1")]
    pub address: IpAddr,
    /// HTTP listening port.
    #[arg(short = 'p', long, default_value = "8080")]
    pub port: u16,
}
