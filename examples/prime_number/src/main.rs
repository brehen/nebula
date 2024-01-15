use shared::{run_function, FunctionType};

fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(prime_number, func_type);
}

fn prime_number(n: u128) -> String {
    format!("{:?}", nth_prime(n))
}

fn is_prime(n: u128) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

fn nth_prime(n: u128) -> u128 {
    let mut primes = Vec::new();

    for i in 1..18446744073709551615 {
        if is_prime(i) {
            primes.push(i);
            if primes.len() - 1 == n as usize {
                return i;
            }
        }
    }

    return 0;
}
