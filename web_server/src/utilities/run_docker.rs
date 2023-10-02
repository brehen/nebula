use axum::{extract::Path, response::Html};
use nebula_lib::docker_runner::run_docker_image;

pub async fn run_fib_docker(Path(size): Path<String>) -> Html<String> {
    let fib_image = "brehen/fibonacci-node";
    let sequence = run_docker_image(fib_image, size.as_str()).expect("To get a sequence back");

    let html = format!(
        "<h1>Fib is nice! Here are the first {} numbers of the sequence:<h1><p>Total time: {}ms</p><p>Startup time: {}ms</p><p>{}</p>",
        size, sequence.total_elapsed_time, sequence.startup_time, sequence.result
    );
    Html(html)
}
