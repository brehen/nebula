pub mod docker_module;
pub mod wasm_module;

#[derive(Debug)]
pub struct FunctionResult {
    pub metrics: Option<Metrics>,
    pub result: String,
}

#[derive(Debug)]
pub struct Metrics {
    pub startup_time: u128,
    pub total_runtime: u128,
}
