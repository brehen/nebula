use std::{
    io::{Error, Result, Write},
    process::{Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crate::models::{FunctionResult, Metrics, ModuleType};

pub fn run_docker_image(image_name: &str, input: &str) -> Result<FunctionResult> {
    let start = Instant::now();

    let mut child = Command::new("docker")
        .args(["run", "--rm", "-i", image_name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let cmd_start = current_ms()?;

    child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;

    let output = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let (result, actual_startup) = parse_output(&stdout, cmd_start)?;

    let total_runtime = start.elapsed().as_millis();

    Ok(FunctionResult {
        result,
        metrics: Some(Metrics {
            startup_time: actual_startup,
            total_runtime,
            startup_percentage: ((actual_startup as f64 / total_runtime as f64) * 100.0).round(),
        }),
        func_type: ModuleType::Docker,
        func_name: image_name.to_string(),
        input: input.to_string(),
    })
}

fn current_ms() -> std::io::Result<u128> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|err| Error::new(std::io::ErrorKind::Other, err))
}

fn parse_output(output: &str, cmd_startup: u128) -> Result<(String, u128)> {
    let mut parts = output.trim().split('|');
    let result = parts.next().ok_or(Error::new(
        std::io::ErrorKind::Other,
        "No result part in output",
    ))?;
    let actual_startup = parts
        .next()
        .ok_or(Error::new(
            std::io::ErrorKind::Other,
            "No timestamp part in output",
        ))?
        .parse::<u128>()
        .map_err(|_| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                "Timestamp is not a valid u128",
            )
        })?;

    Ok((result.to_string(), actual_startup - cmd_startup))
}
