use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(|| async { "<!DOCTYPE html><html><body><div>Hei Simen!<br><br>Hvordan går det? :)</div></body></html>" }),
    );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
