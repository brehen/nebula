use shared::{run_function, FunctionType};

// Reads std in as input, retrieves the fibonacci sequence and returns the last number of the
// fibonacci sequence of the provided size
fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(fibonacci, func_type);
}

fn fibonacci(size: i32) -> String {
    let sequence = compute_fibonacci(size);
    format!("{:?}", sequence.last().unwrap_or(&0))
}

fn compute_fibonacci(size: i32) -> Vec<u64> {
    let mut sequence = Vec::<u64>::new();

    for i in 0..size {
        let j = i as usize;
        if i == 0 || i == 1 {
            sequence.push(i as u64);
        } else {
            let next_value = sequence[j - 1] + sequence[j - 2];
            sequence.push(next_value);
        }
    }

    sequence
}
