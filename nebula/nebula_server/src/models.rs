use askama::Template;
use nebula_lib::models::{FunctionResult, ModuleType};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::utilities::format::format_micro_to_milli;

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
    pub avg_wasm_runtime: u128,
    pub avg_docker_startup: u128,
    pub avg_docker_runtime: u128,
    pub avg_docker_total_time: u128,
}

impl FCList {
    fn format_time(&self, time: &u128) -> String {
        format_micro_to_milli(*time)
    }
}

#[derive(Deserialize, Clone)]
pub struct FunctionRequest {
    pub function_name: String,
    pub input: String,
    pub module_type: ModuleType,
    #[serde(default = "default_num_calls")]
    pub num_calls: u8,
    #[serde(default = "default_image")]
    pub base_image: String,
}

fn default_num_calls() -> u8 {
    1
}

fn default_image() -> String {
    "debian".to_string()
}

pub fn verify_image(image: &str) -> bool {
    let valid_images = ["debian", "ubuntu", "archlinux"];
    valid_images.contains(&image)
}
