use std::{
    io::{Result, Write},
    process::{Command, Stdio},
    time::Instant,
};

use crate::models::ModuleType;

use super::{FunctionResult, Metrics};

pub struct DockerModule<Input = String, Output = Option<String>> {
    pub image_name: String,
    pub input: Input,
    pub output: Option<Output>,
    pub collect_metrics: Option<bool>,
    pub metrics: Option<Metrics>,
}

#[allow(dead_code)]
impl<Input, Output> DockerModule<Input, Output> {
    fn new(image_name: String, input: Input, collect_metrics: Option<bool>) -> Self {
        DockerModule {
            image_name,
            input,
            output: None,
            metrics: None,
            collect_metrics,
        }
    }

    fn set_output(&mut self, output: Output) {
        self.output = Some(output);
    }

    pub fn should_collect_metrics(&self) -> bool {
        self.collect_metrics.unwrap_or(false)
    }

    fn call(&mut self) -> Result<FunctionResult> {
        let mut startup_time: Option<u128> = None;
        let mut total_runtime: Option<u128> = None;
        let start_time = if self.should_collect_metrics() {
            Some(Instant::now())
        } else {
            None
        };
        let mut child = Command::new("docker")
            .args(["run", "--rm", "-i", self.image_name.as_str()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(start_time) = start_time {
            startup_time = Some(start_time.clone().elapsed().as_millis());
        };

        println!("{:?}", startup_time);

        child
            .stdin
            .as_mut()
            .unwrap()
            // .write_all(self.input.as_bytes())
            .write_all(b"5\n")
            .expect("to be able to write to stdin");

        let output = child.wait_with_output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(start_time) = start_time {
            total_runtime = Some(start_time.elapsed().as_millis());
        }

        let metrics = match startup_time.is_some() && total_runtime.is_some() {
            true => Some(Metrics {
                startup_time: startup_time.unwrap(),
                total_runtime: total_runtime.unwrap(),
                startup_percentage: (startup_time.unwrap() as f64 / total_runtime.unwrap() as f64)
                    * 100.0,
            }),
            false => None,
        };

        Ok(FunctionResult {
            result: stdout.trim().to_string(),
            metrics,
            func_type: ModuleType::Docker,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const IMAGE_NAME_EXAMPLE: &str = "docker/test_test";
    #[test]
    fn test_create_docker_module() {
        let module = DockerModule::<String, String>::new(
            IMAGE_NAME_EXAMPLE.to_string(),
            "5".to_string(),
            None,
        );
        assert_eq!(module.image_name, IMAGE_NAME_EXAMPLE)
    }
}
