use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let num: usize = buffer.trim().parse().unwrap();
    let fib_seq = fibonacci(num);
    println!("{:?}", fib_seq);
}

fn fibonacci(n: usize) -> Vec<u64> {
    let mut seq = vec![0, 1];
    for i in 2..n {
        let next = seq[i - 1] + seq[i - 2];
        seq.push(next);
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sequence = fibonacci(5);
        assert_eq!(sequence, vec![0, 1, 1, 2, 3]);

        let sequence = fibonacci(9);
        assert_eq!(sequence, vec![0, 1, 1, 2, 3, 5, 8, 13, 21]);
    }
}
