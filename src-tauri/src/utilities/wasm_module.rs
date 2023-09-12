use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FunctionResult {
    pub function_name: String,
    pub total_runtime: u128,
}

impl FunctionResult {
    pub fn get_runtime(&self) -> u128 {
        self.total_runtime
    }
}
