use std::env;

use axum::{extract::Path, response::Html};
use nebula_lib::wasm_runner::{self, FunctionResult};

pub fn run_wasm_module(size: &str) -> FunctionResult {
    let cwd = env::current_dir().expect("cwd to exist");
    let file_path = format!(
        "{}/projects/wasm_modules/fib.wasm",
        cwd.to_str().expect("to parse to str")
    );
    println!("{:?}", file_path);
    wasm_runner::run_wasi_module(file_path.as_str(), size).expect("There to be a proper return")
}

pub async fn run_fib_module(Path(size): Path<String>) -> Html<String> {
    let sequence = run_wasm_module(size.as_str());
    let html = format!(
        "<h1>Fib is nice! Here are the first {} numbers of the sequence:<h1><p>Total time:{}ms</p><p>Startup time:{}ms</p>",
        size, sequence.total_elapsed_time, sequence.startup_time
    );
    Html(html)
}
