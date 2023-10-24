use axum::{extract::Path, response::Html};
use nebula_lib::wasm_runner::{self};
use serde::Deserialize;

use super::get_file_path::get_file_path;

pub async fn run_fib_module(Path(size): Path<String>) -> Html<String> {
    let file_path = get_file_path("fibonacci");
    let add_path = get_file_path("add");

    let sequence =
        wasm_runner::run_wasi_module(&file_path, "5").expect("There to be a proper return");
    let sequence2 =
        wasm_runner::run_wasi_module(&file_path, "8").expect("There to be a proper return");

    dbg!(format!("{},{}", sequence.result, sequence2.result).as_str());
    let number = wasm_runner::run_wasi_module(
        &add_path,
        format!("{},{}", sequence.result, sequence2.result).as_str(),
    )
    .expect("There to be a proper return");

    println!("number was: {}", number.result);

    let mut html = format!("<h1>Fibonacci</h1>");
    if let Some(metrics) = sequence.metrics {
        html = format!("<h1>Fib is nice! Here are the first {} numbers of the sequence:<h1><p>Total time: {}ms</p><p>Startup time: {}ms</p><p>{}</p>",
        size, metrics.total_runtime, metrics.startup_time, sequence.result);
    };

    Html(html)
}

#[derive(Deserialize)]
pub struct Params {
    module: String,
    input: String,
}

pub async fn run_wasm_module(Path(Params { module, input }): Path<Params>) -> Html<String> {
    let file_path = get_file_path(&module);
    match wasm_runner::run_wasi_module(&file_path, &input) {
        Ok(result) => Html(format!(
            "<div><h1>{}, input: {}</h1><p>Result was: {}</p></div>",
            module, input, result.result
        )),
        Err(err) => Html(format!(
            "<p>Whoops! Error!</p> <p style=\"color:red;\">{}</p>",
            err.to_string()
        )),
    }
}
