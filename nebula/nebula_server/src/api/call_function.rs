use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Form};
use nebula_lib::{
    docker_runner::run_docker_image,
    models::{FunctionResult, ModuleType},
    wasm_runner::run_wasi_module,
};
use tracing::info;

use crate::{
    models::{AppState, FCList, FunctionRequest},
    utilities::{get_file_path::get_file_path, html_template::HtmlTemplate, persist::save_results},
};

pub async fn call_function(
    State(state): State<Arc<AppState>>,
    Form(request): Form<FunctionRequest>,
) -> impl IntoResponse {
    info!(
        "calling function: {:?}, type: {:?}, {} times",
        request.function_name, request.module_type, request.num_calls
    );

    let mut results = Vec::new();

    for _ in 0..request.num_calls {
        let req = request.clone();
        let result: FunctionResult = match request.module_type {
            ModuleType::Docker => {
                let docker_module =
                    format!("nebula-function-{}-{}", req.function_name, req.base_image);
                run_docker_image(
                    &docker_module,
                    &req.input,
                    req.function_name,
                    req.base_image,
                )
                .expect("It to work")
            }
            ModuleType::Wasm => {
                let function_path = get_file_path(&req.function_name);
                run_wasi_module(&function_path, &req.input, req.function_name).expect("to work")
            }
        };

        results.push(result);
    }

    let mut lock = state.function_calls.lock().await;

    for result in results {
        lock.push(result);
    }

    let function_results: Vec<FunctionResult> = lock.clone().into_iter().rev().collect();
    let _ = save_results(function_results.clone());

    let template = get_fc_list(function_results);

    HtmlTemplate(template)
}

pub fn get_fc_list(function_results: Vec<FunctionResult>) -> FCList {
    let total_invocations = function_results.len();
    let results = if total_invocations == 0 {
        FCList {
            function_results,
            total_wasm_invocations: 0,
            total_docker_invocations: 0,
            avg_wasm_startup: 0,
            avg_docker_startup: 0,
            avg_wasm_runtime: 0,
            avg_docker_runtime: 0,
            avg_wasm_total_time: 0,
            avg_docker_total_time: 0,
        }
    } else {
        let wasm_results: Vec<FunctionResult> = function_results
            .clone()
            .into_iter()
            .filter(|result| matches!(result.func_type, ModuleType::Wasm))
            .collect();

        let total_wasm_invocations = wasm_results.len();
        let wasm_startup_times: u128 = wasm_results
            .iter()
            .map(|result| result.metrics.as_ref().unwrap().startup_time)
            .sum();
        let wasm_runtime_sum: u128 = wasm_results
            .iter()
            .map(|result| {
                let metrics = result.metrics.as_ref().unwrap();
                metrics.total_runtime - metrics.startup_time
            })
            .sum();
        let wasm_total_time_sum: u128 = wasm_results
            .iter()
            .map(|result| result.metrics.as_ref().unwrap().total_runtime)
            .sum();

        let docker_results: Vec<FunctionResult> = function_results
            .clone()
            .into_iter()
            .filter(|result| matches!(result.func_type, ModuleType::Docker))
            .collect();

        let total_docker_invocations = docker_results.len();
        let docker_startup_times: u128 = docker_results
            .iter()
            .map(|result| result.metrics.as_ref().unwrap().startup_time)
            .sum();
        let docker_runtime_sum: u128 = docker_results
            .iter()
            .map(|result| {
                let metrics = result.metrics.as_ref().unwrap();
                metrics.total_runtime - metrics.startup_time
            })
            .sum();
        let docker_total_time_sum: u128 = docker_results
            .iter()
            .map(|result| result.metrics.as_ref().unwrap().total_runtime)
            .sum();

        FCList {
            function_results,
            total_wasm_invocations,
            total_docker_invocations,
            avg_wasm_startup: wasm_startup_times / (total_wasm_invocations as u128).max(1),
            avg_wasm_runtime: wasm_runtime_sum / (total_wasm_invocations as u128).max(1),
            avg_wasm_total_time: wasm_total_time_sum / (total_wasm_invocations as u128).max(1),
            avg_docker_startup: docker_startup_times / (total_docker_invocations as u128).max(1),
            avg_docker_runtime: docker_runtime_sum / (total_docker_invocations as u128).max(1),
            avg_docker_total_time: docker_total_time_sum
                / (total_docker_invocations as u128).max(1),
        }
    };

    results
}
