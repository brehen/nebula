use shared::{run_function, FunctionType};

fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(factorial, func_type);
}

fn factorial(num: u128) -> String {
    let fact: u128 = (1..=num).product();
    format!("{:?}", fact)
}
