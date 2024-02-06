use askama::Template;
use nebula_lib::models::{FunctionResult, ModuleType};
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub function_calls: Mutex<Vec<FunctionResult>>,
}

#[derive(Template, Debug)]
#[template(path = "components/function_results.rs.html")]
pub struct FCList {
    pub function_results: Vec<FunctionResult>,
    pub total_wasm_invocations: usize,
    pub total_docker_invocations: usize,
    pub avg_wasm_startup: u128,
    pub avg_wasm_total_time: u128,
    pub avg_docker_startup: u128,
    pub avg_docker_total_time: u128,
}

#[derive(Deserialize, Clone)]
pub struct FunctionRequest {
    pub function_name: String,
    pub input: String,
    pub module_type: ModuleType,
    #[serde(default = "default_num_calls")]
    pub num_calls: u8,
}

fn default_num_calls() -> u8 {
    1
}
