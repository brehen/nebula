pub fn fibonacci(size: usize) -> Vec<u64> {
    let mut sequence = Vec::with_capacity(size);

    for i in 0..size {
        if i == 0 || i == 1 {
            sequence.push(i as u64);
        } else {
            let next_value = sequence[i - 1] + sequence[i - 2];
            sequence.push(next_value);
        }
    }

    println!("Help me");

    sequence
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

