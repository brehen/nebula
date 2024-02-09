use anyhow::{Context, Error, Result};

use std::{
    any::type_name,
    fmt::Display,
    io::{stdin, BufRead},
    str::FromStr,
};

mod docker {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn get_epoch_timestamp() -> Option<String> {
        let now = SystemTime::now();
        let dur_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let msecs_since_epoch = dur_since_epoch.as_micros();
        Some(format!("{}", msecs_since_epoch))
    }
}

mod wasm {
    pub fn get_epoch_timestamp() -> Option<String> {
        None
    }
}

/// Utility function for getting stdin and parse it to expected type, up to caller to decide
/// Usage:
/// ```rust
/// let input: i32 = get_stdin().expect("Failed to parse stdin, check value");
/// ```

fn get_stdin<T: FromStr>() -> Result<T> {
    let mut input = String::new();

    stdin()
        .lock()
        .read_line(&mut input)
        .context("Failed to read line")?;

    input
        .trim()
        .parse::<T>()
        .map_err(|_| Error::msg(format!("Failed to parse input as {}", type_name::<T>())))
}

fn print_stdout<T: Display>(output: T, timestamp: Option<String>) {
    match timestamp {
        Some(timestamp) => println!("{}|{}", output, timestamp),
        None => println!("{}", output),
    }
}

pub enum FunctionType {
    Docker,
    Wasm,
}

pub fn run_function<T, F, R>(func: F, func_type: FunctionType)
where
    T: FromStr,
    F: Fn(T) -> R,
    R: std::fmt::Display,
{
    let timestamp = match func_type {
        FunctionType::Docker => docker::get_epoch_timestamp(),
        FunctionType::Wasm => wasm::get_epoch_timestamp(),
    };

    let input: T = get_stdin().expect("To parse correctly");
    let result = func(input);
    print_stdout(result, timestamp)
}
