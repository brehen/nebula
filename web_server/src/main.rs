use axum::{routing::get, Router};
use nebula_lib::docker_runner::run_docker_image;
use nebula_server::utilities::{run_docker::run_fib_docker, run_wasm_module::run_fib_module};

#[tokio::main]
async fn main() {
    // let fib_image = "brehen/fibonacci-node";
    // let sequence = run_docker_image(fib_image).expect("To get a sequence back");
    // println!("whoa {:?}", sequence);
    // build our application with a single route
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

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
