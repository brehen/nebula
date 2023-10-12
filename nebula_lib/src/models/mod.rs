use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

pub mod docker_module;
pub mod wasm_module;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResult {
    pub metrics: Option<Metrics>,
    pub result: String,
}

impl Display for FunctionResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(ref metrics) = self.metrics {
            write!(f, "Result was: {}\n{}", self.result, metrics)
        } else {
            write!(f, "Result was: {}", self.result)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub startup_time: u128,
    pub total_runtime: u128,
}

impl Display for Metrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Function took {}ms to startup and total runtime: {}",
            self.startup_time, self.total_runtime
        )
    }
}
