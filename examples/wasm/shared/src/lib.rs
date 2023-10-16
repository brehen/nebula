use anyhow::{Context, Error, Result};
use std::{
    any::type_name,
    io::{stdin, BufRead},
    str::FromStr,
};

/// Utility function for getting stdin and parse it to expected type, up to caller to decide
/// Usage:
/// ```rust
/// let input: i32 = get_stdin().expect("Failed to parse stdin, check value");
/// ```

pub fn get_stdin<T: FromStr>() -> Result<T> {
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
