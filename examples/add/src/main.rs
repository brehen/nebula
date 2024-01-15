use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut input = String::new();

    stdin
        .lock()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut iter = input.trim().split(',');
    let a = iter.next().and_then(|s| s.parse::<i32>().ok());
    let b = iter.next().and_then(|s| s.parse::<i32>().ok());

    match (a, b) {
        (Some(a), Some(b)) => {
            println!("{}", add(a, b));
        }
        _ => {
            println!("Wrong input, expected 2 numbers separated by a comma!");
        }
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
