use std::{
    io::{Result, Write},
    process::{Command, Stdio},
    time::Instant,
};

use crate::models::{FunctionResult, Metrics};

pub fn run_docker_image(image_name: &str, input: &str) -> Result<FunctionResult> {
    let start = Instant::now();
    let mut child = Command::new("docker")
        .args(["run", "--rm", "-i", image_name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let startup_time = start.clone().elapsed().as_millis();

    child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;

    let output = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let total_runtime = start.elapsed().as_millis();
    Ok(FunctionResult {
        result: stdout.trim().to_string(),
        metrics: Some(Metrics {
            startup_time,
            total_runtime,
        }),
    })
}
