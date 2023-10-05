use std::env;

use axum::{extract::Path, response::Html};
use nebula_lib::{
    models::FunctionResult,
    wasm_runner::{self},
};

pub fn run_wasm_module(size: &str) -> FunctionResult {
    let cwd = env::current_dir().expect("cwd to exist");
    let cwd = cwd.to_str().expect("To parse");
    // let cwd = "/Users/mariuskluften";
    let file_path = format!("{}/projects/wasm_modules/fib.wasm", cwd);
    println!("{:?}", file_path);
    wasm_runner::run_wasi_module(file_path.as_str(), size).expect("There to be a proper return")
}

pub async fn run_fib_module(Path(size): Path<String>) -> Html<String> {
    let sequence = run_wasm_module(size.as_str());
    let mut html = format!("<h1>Fibonacci</h1>");
    if let Some(metrics) = sequence.metrics {
        html = format!("<h1>Fib is nice! Here are the first {} numbers of the sequence:<h1><p>Total time: {}ms</p><p>Startup time: {}ms</p><p>{}</p>",
        size, metrics.total_runtime, metrics.startup_time, sequence.result);
    };
    Html(html)
}
