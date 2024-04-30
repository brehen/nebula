use std::collections::HashMap;

use super::request::FunctionResult;

#[derive(Default, Debug, Clone, Copy)]
pub struct Metrics {
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub average: f64,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            min: f64::MAX,
            max: f64::MIN,
            median: 0.0,
            average: 0.0,
        }
    }
}

#[derive(Default)]
pub struct NormalizedMetrics {
    pub startup_time: Metrics,
    pub total_runtime: Metrics,
    pub average_power: Metrics,
    pub average_power_isolated: Metrics,
    pub energy_consumption_wh: Metrics,
    pub energy_consumption_isolated_wh: Metrics,
    pub num_invoked: u32,
}

pub fn analyze_power_data(
    measurements: &[FunctionResult],
) -> HashMap<(String, String, String), NormalizedMetrics> {
    let mut normalized_metrics = HashMap::new();

    for measurement in measurements {
        let func_type = match measurement.func_type {
            super::request::ModuleType::Docker => "Docker",
            super::request::ModuleType::Wasm => "Wasm",
        };
        let key = (
            func_type.to_owned(),
            measurement.func_name.clone(),
            measurement.input.clone(),
        );
        let metrics = normalized_metrics
            .entry(key)
            .or_insert_with(NormalizedMetrics::default);

        metrics.num_invoked += 1;

        if let Some(measurement_metrics) = &measurement.metrics {
            update_min_max_avg(
                &mut metrics.startup_time,
                measurement_metrics.startup_time as f64,
            );
            update_min_max_avg(
                &mut metrics.total_runtime,
                measurement_metrics.total_runtime as f64,
            );
            if let Some(average_power) = measurement_metrics.average_power {
                update_min_max_avg(&mut metrics.average_power, average_power as f64);
            }
            if let Some(average_power_isolated) = measurement_metrics.average_power_isolated {
                update_min_max_avg(
                    &mut metrics.average_power_isolated,
                    average_power_isolated as f64,
                );
            }
            if let Some(energy_consumption_wh) = measurement_metrics.energy_consumption_wh {
                update_min_max_avg(
                    &mut metrics.energy_consumption_wh,
                    energy_consumption_wh as f64,
                );
            }
            if let Some(energy_consumption_isolated_wh) =
                measurement_metrics.energy_consumption_isolated_wh
            {
                update_min_max_avg(
                    &mut metrics.energy_consumption_isolated_wh,
                    energy_consumption_isolated_wh as f64,
                );
            }
        }
    }

    calculate_medians(&mut normalized_metrics, measurements);

    normalized_metrics
}

fn update_min_max_avg(metrics: &mut Metrics, value: f64) {
    if metrics.min == 0.0 {
        metrics.min = value;
    } else {
        metrics.min = metrics.min.min(value);
    }
    metrics.max = metrics.max.max(value);
    metrics.average += value;
}

fn calculate_medians(
    normalized_metrics: &mut HashMap<(String, String, String), NormalizedMetrics>,
    measurements: &[FunctionResult],
) {
    for (_, metrics) in normalized_metrics.iter_mut() {
        metrics.startup_time.median = calculate_median(measurements, |m| {
            m.metrics
                .as_ref()
                .map(|mm| mm.startup_time as f64)
                .unwrap_or_default()
        });
        metrics.total_runtime.median = calculate_median(measurements, |m| {
            m.metrics
                .as_ref()
                .map(|mm| mm.total_runtime as f64)
                .unwrap_or_default()
        });
        metrics.average_power.median = calculate_median(measurements, |m| {
            m.metrics
                .as_ref()
                .map(|mm| mm.average_power.unwrap() as f64)
                .unwrap_or_default()
        });
        metrics.average_power_isolated.median = calculate_median(measurements, |m| {
            m.metrics
                .as_ref()
                .map(|mm| mm.average_power_isolated.unwrap() as f64)
                .unwrap_or_default()
        });
        metrics.energy_consumption_wh.median = calculate_median(measurements, |m| {
            m.metrics
                .as_ref()
                .map(|mm| mm.energy_consumption_wh.unwrap() as f64)
                .unwrap_or_default()
        });
        metrics.energy_consumption_isolated_wh.median = calculate_median(measurements, |m| {
            m.metrics
                .as_ref()
                .map(|mm| mm.energy_consumption_isolated_wh.unwrap() as f64)
                .unwrap_or_default()
        });
        metrics.startup_time.average /= metrics.num_invoked as f64;
        metrics.total_runtime.average /= metrics.num_invoked as f64;
        metrics.average_power.average /= metrics.num_invoked as f64;
        metrics.average_power_isolated.average /= metrics.num_invoked as f64;
        metrics.energy_consumption_wh.average /= metrics.num_invoked as f64;
        metrics.energy_consumption_isolated_wh.average /= metrics.num_invoked as f64;
    }
}

fn calculate_median<F>(measurements: &[FunctionResult], f: F) -> f64
where
    F: Fn(&FunctionResult) -> f64,
{
    let mut values: Vec<_> = measurements.iter().map(f).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    }
}

pub fn print_analyzed_power_metrics(
    metrics_map: &HashMap<(String, String, String), NormalizedMetrics>,
) {
    let mut keys: Vec<&(String, String, String)> = metrics_map.keys().clone().collect();

    keys.sort_unstable_by_key(|&key| key.2.parse::<u32>().unwrap());
    println!("{:?}", keys);

    let keys: Vec<_> = keys.iter().take(10).collect();

    for key in keys {
        let metrics = metrics_map.get(key).unwrap();
        println!("Function Type: {}", key.0);
        println!("Function Name: {}", key.1);
        println!("Input: {}", key.2);
        println!("Invoked: {}", metrics.num_invoked);
        println!(
            "Startup Time - Min: {:.2}ms, Max: {:.2}ms, Median: {:.2}ms, Average: {:.2}ms",
            metrics.startup_time.min / 1000.0,
            metrics.startup_time.max / 1000.0,
            metrics.startup_time.median / 1000.0,
            metrics.startup_time.average / 1000.0
        );
        println!(
            "Total Runtime - Min: {:.2}ms, Max: {:.2}ms, Median: {:.2}ms, Average: {:.2}ms",
            metrics.total_runtime.min / 1000.0,
            metrics.total_runtime.max / 1000.0,
            metrics.total_runtime.median / 1000.0,
            metrics.total_runtime.average / 1000.0
        );
        println!(
            "Average Power - Min: {:.2}W, Max: {:.2}W, Median: {:.2}W, Average: {:.2}W",
            metrics.average_power.min,
            metrics.average_power.max,
            metrics.average_power.median,
            metrics.average_power.average
        );
        println!(
            "Average Power Isolated - Min: {:.3}W, Max: {:.3}W, Median: {:.4}W, Average: {:.4}W",
            metrics.average_power_isolated.min,
            metrics.average_power_isolated.max,
            metrics.average_power_isolated.median,
            metrics.average_power_isolated.average
        );
        println!(
            "Energy Consumption WH - Min: {:.9}μWh, Max: {:.9}μWh, Median: {:.9}μWh, Average: {:.9}μWh",
            metrics.energy_consumption_wh.min * 1_000_000.0,
            metrics.energy_consumption_wh.max * 1_000_000.0,
            metrics.energy_consumption_wh.median * 1_000_000.0,
            metrics.energy_consumption_wh.average * 1_000_000.0,
        );
        println!(
            "Energy Consumption Isolated WH - Min: {:.9}μWh, Max: {:.9}μWh, Median: {:.9}μWh, Average: {:.9}μWh",
            metrics.energy_consumption_isolated_wh.min * 1_000_000.0,
            metrics.energy_consumption_isolated_wh.max * 1_000_000.0,
            metrics.energy_consumption_isolated_wh.median * 1_000_000.0,
            metrics.energy_consumption_isolated_wh.average * 1_000_000.0
        );
        println!("---------------------------------------");
    }
}
