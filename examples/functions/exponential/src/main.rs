use shared::{run_function, FunctionType};

fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(exponential, func_type);
}

fn exponential(x: f64) -> String {
    let exp: f64 = f64::exp(x);
    format!("{:?}", exp)
}
