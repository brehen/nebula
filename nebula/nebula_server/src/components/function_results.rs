use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse, Form};
use nebula_lib::{
    docker_runner::run_docker_image,
    models::{FunctionResult, ModuleType},
    wasm_runner::run_wasi_module,
};
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::info;

use crate::utilities::{
    get_file_path::get_file_path, html_template::HtmlTemplate, persist::save_results,
};

#[derive(Debug)]
pub struct AppState {
    pub function_calls: Mutex<Vec<FunctionResult>>,
}

#[derive(Template, Debug)]
#[template(path = "components/function_results.rs.html")]
struct FCList {
    function_results: Vec<FunctionResult>,
    total_invocations: usize,
    avg_startup: u128,
    avg_total_time: u128,
}

#[derive(Deserialize)]
pub struct FunctionRequest {
    function_name: String,
    input: String,
    module_type: ModuleType,
}

pub async fn call_function(
    State(state): State<Arc<AppState>>,
    Form(request): Form<FunctionRequest>,
) -> impl IntoResponse {
    info!(
        "calling function: {:?}, type: {:?}",
        request.function_name, request.module_type
    );
    let result: FunctionResult = match request.module_type {
        ModuleType::Docker => {
            let docker_module = format!("nebula-function-{}", request.function_name);
            run_docker_image(&docker_module, &request.input, request.function_name)
                .expect("It to work")
        }
        ModuleType::Wasm => {
            let function_path = get_file_path(&request.function_name);
            run_wasi_module(&function_path, &request.input, request.function_name).expect("to work")
        }
    };
    let mut lock = state.function_calls.lock().await;
    lock.push(result);

    let function_results: Vec<FunctionResult> = lock.clone().into_iter().rev().collect();
    let _ = save_results(function_results.clone());
    let total_invocations = function_results.len();
    let startup_time_sum: u128 = function_results
        .iter()
        .map(|result| result.metrics.as_ref().unwrap().startup_time)
        .sum();
    let runtime_sum: u128 = function_results
        .iter()
        .map(|result| result.metrics.as_ref().unwrap().total_runtime)
        .sum();

    let template = FCList {
        function_results,
        total_invocations,
        avg_startup: startup_time_sum / total_invocations as u128,
        avg_total_time: runtime_sum / total_invocations as u128,
    };

    HtmlTemplate(template)
}

pub async fn get_function_results(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lock = state.function_calls.lock().await;
    let function_results: Vec<FunctionResult> = lock.clone().into_iter().rev().collect();

    let total_invocations = function_results.len();
    let template;
    if total_invocations == 0 {
        template = FCList {
            function_results,
            total_invocations,
            avg_startup: 0,
            avg_total_time: 0,
        }
    } else {
        let startup_time_sum: u128 = function_results
            .iter()
            .map(|result| result.metrics.as_ref().unwrap().startup_time)
            .sum();
        let runtime_sum: u128 = function_results
            .iter()
            .map(|result| result.metrics.as_ref().unwrap().total_runtime)
            .sum();

        template = FCList {
            function_results,
            total_invocations,
            avg_startup: startup_time_sum / total_invocations as u128,
            avg_total_time: runtime_sum / total_invocations as u128,
        };
    }

    HtmlTemplate(template)
}
