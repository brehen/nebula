use std::io::{stdin, BufRead};

// Reads std in as input, retrieves the fibonacci sequence and returns the last number of the
// fibonacci sequence of the provided size
fn main() {
    let stdin = stdin();
    let mut input = String::new();

    stdin
        .lock()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parsed_input: Result<i32, _> = input.trim().parse();

    let size = if let Ok(size) = parsed_input {
        size
    } else {
        println!("Expected a number, got something else! Defaulting to 1");
        1
    };

    let sequence = fibonacci(size);
    println!("{:?}", sequence.last().unwrap());
}

fn fibonacci(size: i32) -> Vec<u64> {
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

    // println!("Help me");

    sequence
}
