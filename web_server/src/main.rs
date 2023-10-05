use std::net::{IpAddr, SocketAddr};

use axum::{routing::get, Router};
use clap::{ArgAction, Parser};
use log::{info, LevelFilter};
use nebula_server::utilities::{run_docker::run_fib_docker, run_wasm_module::run_fib_module};

#[tokio::main]
async fn main() {
    let options = Options::parse();
    env_logger::Builder::new()
        .filter_level(options.log_level())
        .init();

    let app = Router::new()
        .route(
            "/",
            get(|| async { "<!DOCTYPE html><html><body><div>Hei Simen!<br><br>Hvordan går det? :)</div></body></html>" }),
        )
        .route(
            "/fib/:size",
            get(run_fib_module),
        )
        .route(
            "/docker/fib/:size",
            get(run_fib_docker),
        );

    info!(
        "Up and running on address {}:{}!",
        options.address, options.port
    );

    // run it with hyper on localhost:3000
    axum::Server::bind(&SocketAddr::new(options.address, options.port))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Options {
    /// Increase logs verbosity (Error (default), Warn, Info, Debug, Trace).
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count)]
    pub log_level: u8,
    /// HTTP listening address.
    #[arg(short = 'a', long, default_value = "127.0.0.1")]
    pub address: IpAddr,
    /// HTTP listening port.
    #[arg(short = 'p', long, default_value = "8080")]
    pub port: u16,
}

impl Options {
    pub fn log_level(&self) -> LevelFilter {
        match self.log_level {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }
}
