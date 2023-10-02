use std::{
    io::{Result, Write},
    process::{Command, Stdio},
    time::Instant,
};

use crate::models::FunctionResult;

pub fn run_docker_image(image_name: &str, size: &str) -> Result<FunctionResult> {
    let start = Instant::now();
    let mut child = Command::new("docker")
        .args(["run", "--rm", "-i", image_name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let startup_time = start.clone().elapsed().as_millis() as usize;

    child.stdin.as_mut().unwrap().write_all(size.as_bytes())?;

    let output = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let total_elapsed_time = start.elapsed().as_millis() as usize;
    Ok(FunctionResult {
        result: stdout.trim().to_string(),
        startup_time,
        total_elapsed_time,
    })
}
