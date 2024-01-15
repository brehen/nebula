use rand::prelude::*;
use shared::{run_function, FunctionType};

fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(estimate_pi, func_type);
}

pub fn estimate_pi(n: u128) -> String {
    println!("Geting here");
    let total = 1_000_000 * n;
    let mut count = 0;
    let mut rng = thread_rng();
    for _ in 1..total {
        let x = (2.0 * rng.gen::<f32>()) - 1.0;
        let y = (2.0 * rng.gen::<f32>()) - 1.0;
        if (x * x + y * y).sqrt() < 1.0 {
            count += 1;
        }
    }
    format!("Estimated Pi: {}", 4.0 * count as f32 / total as f32)
}
