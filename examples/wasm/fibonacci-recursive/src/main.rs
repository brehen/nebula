use shared::get_stdin;

// Reads std in as input, retrieves the fibonacci sequence and returns the last number of the
// fibonacci sequence of the provided size
fn main() {
    let size: i64 = get_stdin().expect("To parse correctly");

    let sequence = fib(size);
    println!("{}", sequence);
}

fn fib(size: i64) -> i64 {
    match size {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}
