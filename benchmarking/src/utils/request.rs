use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::{fmt, time::Duration};
use tokio::time::{sleep, Instant};

use crate::utils::file::write_results;

// const URL: &str = "http://192.168.68.69/api/wasm_headless";
// const URL: &str = "http://raspberrypi.local/api/wasm";
// const URL: &str = "http://nebula.no/api/wasm_headless";

pub async fn bombard_nebula(client: Client, url: &str) -> anyhow::Result<Vec<FunctionResult>> {
    // name, max input value before it breaks, increments of input
    let modules: Vec<(&str, u32, u32)> = vec![
        // ("exponential", 706, 2), // 709 is max
        // ("factorial", 130, 1),   // 130 is max
        ("fibonacci", 300, 5),     // max is very high, but 300 is fine
        ("prime-number", 600, 10), // max is very high, but 600 is fine
    ];
    // let base_images = ["debian", "ubuntu", "archlinux"];
    // let duration_per_function = Duration::new(60, 0);
    //
    let timer = Instant::now();

    let mut function_results: Vec<FunctionResult> = Vec::new();

    for module in modules {
        println!(
            "Measuring input from {} to {}, in incr of {}",
            0, module.1, module.2
        );
        for input_value in 0..=module.1 {
            // do each input value 5 times to account for invariance
            let input_value = input_value * module.2;
            println!(
                "Fetching input value {} for module {}",
                input_value, module.0
            );
            let elapsed = timer.elapsed();
            println!(
                "Benchmark has been running for {}h:{}m:{}s now",
                elapsed.as_secs() / 3600,
                elapsed.as_secs() / 60,
                elapsed.as_secs() % 60
            );
            for _ in 0..5 {
                let wasm_results =
                    make_request(&client, url, module.0, "Wasm", &input_value.to_string(), "")
                        .await?;
                function_results.extend(wasm_results);
                sleep(Duration::from_millis(10)).await;
                let request_results = make_request(
                    &client,
                    url,
                    module.0,
                    "Docker",
                    &input_value.to_string(),
                    "debian",
                )
                .await?;
                function_results.extend(request_results);
                sleep(Duration::from_millis(10)).await;
            }
        }
        write_results(
            &function_results,
            format!("temp_{}_results", module.0).as_str(),
        )
        .await?;
    }

    Ok(function_results)
}

async fn make_request(
    client: &Client,
    url: &str,
    module: &str,
    module_type: &str,
    input_value: &str,
    base_image: &str,
) -> reqwest::Result<Vec<FunctionResult>> {
    let payload = [
        ("function_name", module),
        ("module_type", module_type),
        ("input", input_value),
        ("base_image", base_image),
    ];

    let resp = client.post(url).form(&payload).send().await?;

    if resp.status().is_success() {
        let response = resp.json::<FunctionResponse>().await?;
        return Ok(response.results);
    }

    let error = resp.error_for_status();
    eprintln!(
        "Request for input {} failed, message: {:?}",
        input_value, error
    );

    Ok(vec![])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResponse {
    results: Vec<FunctionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResult {
    pub metrics: Option<Metrics>,
    pub result: String,
    pub func_type: ModuleType,
    pub func_name: String,
    pub input: String,
    pub base_image: String,
}

impl fmt::Display for FunctionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metrics = self.metrics.as_ref().unwrap();
        let startup_time_ms = metrics.startup_time as f32 / 1000.0;
        let runtime_ms = metrics.total_runtime - metrics.startup_time;
        let total_time_ms = metrics.total_runtime as f32 / 1000.0;
        let average_power = metrics.average_power.unwrap_or(0.0);
        let average_power_isolated = metrics.average_power_isolated.unwrap_or(0.0);
        let energy_consumption_wh = metrics.energy_consumption_wh.unwrap_or(0.0);
        let energy_consumption_isolated_wh = metrics.energy_consumption_isolated_wh.unwrap_or(0.0);

        let func_type = match self.func_type {
            ModuleType::Docker => "Docker",
            ModuleType::Wasm => "Wasm",
        };

        write!(
            f,
            "Function: {} ({}) => {} ({})\n\
             Startup: {}ms, Runtime: {}ms, Total time: {}ms\n\
             Energy:\n\
             \tAvg draw: {:.2}W/{:.2} (Function/Total Load)\n\
             \tFunction consumed: {:.9}μWh / Total load: {:.9}μWh",
            self.func_name,
            self.input,
            self.result,
            func_type,
            startup_time_ms,
            runtime_ms / 1000,
            total_time_ms,
            average_power_isolated,
            average_power,
            energy_consumption_isolated_wh * 1_000_000.0,
            energy_consumption_wh * 1_000_000.0
        )
    }
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub enum ModuleType {
    Docker,
    Wasm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub startup_time: u128,
    pub start_since_epoch: u128,
    pub total_runtime: u128,
    pub end_since_epoch: u128,
    pub startup_percentage: f32,
    pub average_power: Option<f32>,
    pub average_power_isolated: Option<f32>,
    pub energy_consumption_wh: Option<f32>,
    pub energy_consumption_isolated_wh: Option<f32>,
}
