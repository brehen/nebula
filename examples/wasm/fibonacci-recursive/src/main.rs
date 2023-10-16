use shared::{run_function, FunctionType};

// Reads std in as input, retrieves the fibonacci sequence and returns the last number of the
// fibonacci sequence of the provided size
fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };
    run_function(fib, func_type)
}

fn fib(size: i64) -> i64 {
    match size {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}
