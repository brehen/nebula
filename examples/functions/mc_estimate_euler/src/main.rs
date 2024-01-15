use rand::prelude::*;
use shared::{run_function, FunctionType};

fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(estimate_euler, func_type);
}

pub fn estimate_euler(n: u128) -> String {
    let mut rng = thread_rng();
    let total = 1_000_000 * n;
    let mut total_selections = 0;

    for _ in 1..total {
        let mut sum = 0f32;
        let mut i = 0;
        total_selections += loop {
            sum += rng.gen::<f32>();
            i += 1;
            if sum > 1f32 {
                break i;
            }
        };
    }
    format!("Estimated e: {}", total_selections as f32 / total as f32)
}
