#[no_mangle]
pub extern "C" fn fibonacci(size: i32) -> Vec<i32> {
    let mut sequence = Vec::<i32>::new();

    for i in 0..size {
        if i == 0 {
            sequence.push(0);
        } else if i == 1 {
            sequence.push(1);
        } else {
            let last = *sequence.last().unwrap();
            let second_last = &sequence[sequence.len() - 2];
            sequence.push(last + second_last)
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
