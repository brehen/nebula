use axum::{routing::get, Router};
use nebula_server::utilities::run_wasm_module::run_fib_module;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async { "<!DOCTYPE html><html><body><div>Hei Simen!<br><br>Hvordan går det? :)</div></body></html>" }),
        )
        .route(
            "/fib/:size",
            get(run_fib_module),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
