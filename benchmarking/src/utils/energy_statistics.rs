use std::{collections::HashMap, str::FromStr};

use serde_derive::Serialize;

use super::{
    calc::{median, std_deviation, update_min_max_avg},
    request::{FunctionResult, ModuleType},
    Metrics,
};

#[derive(Serialize, Default, Debug)]
pub struct EnergyMetrics {
    pub startup_time: Metrics,
    pub runtime: Metrics,
    pub average_power: Metrics,
    pub average_power_isolated: Metrics,
    pub energy_consumption_wh: Metrics,
    pub energy_consumption_isolated_wh: Metrics,
    pub num_invoked: u32,
}

pub fn analyze_power_data(
    measurements: &[FunctionResult],
) -> HashMap<(String, String, String), EnergyMetrics> {
    let mut energy_metrics = HashMap::new();

    println!("a: {}", measurements.len());

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
        let metrics = energy_metrics
            .entry(key)
            .or_insert_with(EnergyMetrics::default);

        metrics.num_invoked += 1;

        if let Some(measurement_metrics) = &measurement.metrics {
            update_min_max_avg(
                &mut metrics.startup_time,
                measurement_metrics.startup_time as f64,
            );
            update_min_max_avg(
                &mut metrics.runtime,
                measurement_metrics.total_runtime as f64 - measurement_metrics.startup_time as f64,
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
                let energy_consumption_wh = energy_consumption_wh * 1000.0 * 1000.0;
                update_min_max_avg(
                    &mut metrics.energy_consumption_wh,
                    energy_consumption_wh as f64,
                );
            }
            if let Some(energy_consumption_isolated_wh) =
                measurement_metrics.energy_consumption_isolated_wh
            {
                let energy_consumption_isolated_wh =
                    energy_consumption_isolated_wh * 1000.0 * 1000.0;
                update_min_max_avg(
                    &mut metrics.energy_consumption_isolated_wh,
                    energy_consumption_isolated_wh as f64,
                );
            }
        }
    }

    calculate_medians(&mut energy_metrics, measurements);

    energy_metrics
}

fn calculate_medians(
    energy_metrics: &mut HashMap<(String, String, String), EnergyMetrics>,
    measurements: &[FunctionResult],
) {
    for ((func_type, func_name, input), metrics) in energy_metrics.iter_mut() {
        let these_measurements: Vec<_> = measurements
            .iter()
            .filter(|m| {
                m.func_type == ModuleType::from_str(func_type).unwrap()
                    && &m.func_name == func_name
                    && &m.input == input
            })
            .collect();
        let startup_times: Vec<_> = these_measurements
            .iter()
            .flat_map(|m| m.metrics.as_ref().map(|mm| mm.startup_time as f64))
            .collect();
        let runtimes: Vec<_> = these_measurements
            .iter()
            .flat_map(|m| {
                m.metrics
                    .as_ref()
                    .map(|mm| (mm.total_runtime - mm.startup_time) as f64)
            })
            .collect();
        let average_powers: Vec<_> = these_measurements
            .iter()
            .flat_map(|m| {
                m.metrics
                    .as_ref()
                    .map(|mm| mm.average_power.unwrap() as f64)
            })
            .collect();
        let average_powers_isolated: Vec<_> = these_measurements
            .iter()
            .flat_map(|m| {
                m.metrics
                    .as_ref()
                    .map(|mm| mm.average_power_isolated.unwrap() as f64)
            })
            .collect();
        let energy_consumption_whs: Vec<_> = these_measurements
            .iter()
            .flat_map(|m| {
                m.metrics
                    .as_ref()
                    .map(|mm| (mm.energy_consumption_wh.unwrap() as f64 * 1000.0 * 1000.0))
            })
            .collect();

        let energy_consumption_isolated_whs: Vec<_> = these_measurements
            .iter()
            .flat_map(|m| {
                m.metrics
                    .as_ref()
                    .map(|mm| (mm.energy_consumption_isolated_wh.unwrap() as f64 * 1000.0 * 1000.0))
            })
            .collect();
        metrics.startup_time.mean /= metrics.num_invoked as f64;
        metrics.startup_time.median = median(&startup_times).unwrap_or(0.0);
        metrics.startup_time.std_deviation = std_deviation(&startup_times).unwrap_or(0.0);

        metrics.runtime.mean /= metrics.num_invoked as f64;
        metrics.runtime.median = median(&runtimes).unwrap_or(0.0);
        metrics.runtime.std_deviation = std_deviation(&runtimes).unwrap_or(0.0);

        metrics.average_power.mean /= metrics.num_invoked as f64;
        metrics.average_power.median = median(&average_powers).unwrap_or(0.0);
        metrics.average_power.std_deviation = std_deviation(&average_powers).unwrap_or(0.0);

        metrics.average_power_isolated.mean /= metrics.num_invoked as f64;
        metrics.average_power_isolated.median = median(&average_powers_isolated).unwrap_or(0.0);
        metrics.average_power_isolated.std_deviation =
            std_deviation(&average_powers_isolated).unwrap_or(0.0);

        metrics.energy_consumption_wh.mean /= metrics.num_invoked as f64;
        metrics.energy_consumption_wh.median = median(&energy_consumption_whs).unwrap_or(0.0);
        metrics.energy_consumption_wh.std_deviation =
            std_deviation(&energy_consumption_whs).unwrap_or(0.0);

        metrics.energy_consumption_isolated_wh.mean /= metrics.num_invoked as f64;
        metrics.energy_consumption_isolated_wh.median =
            median(&energy_consumption_isolated_whs).unwrap_or(0.0);
        metrics.energy_consumption_isolated_wh.std_deviation =
            std_deviation(&energy_consumption_isolated_whs).unwrap_or(0.0);
    }
}

pub fn print_analyzed_power_metrics(
    metrics_map: &HashMap<(String, String, String), EnergyMetrics>,
) {
    // let five_inv: Vec<_> = metrics_map
    //     .iter()
    //     .filter(|((_func_name, _func_type, _input), m)| {
    //         // println!("{}, {}, {}", func_name, func_type, input);
    //         m.num_invoked == 0
    //     })
    //     .collect();
    //
    // println!("{:?}", five_inv.len());
    //
    // let mut keys: Vec<&(String, String, String)> = metrics_map
    //     .iter()
    //     .filter(|((_, func_name, _), _m)| func_name == "fibonacci")
    //     .map(|(k, _)| k)
    //     .collect();
    //
    // keys.sort_unstable_by_key(|&key| key.2.parse::<u32>().unwrap());
    //
    // for i in 0..300 {
    //     let koys: Vec<_> = keys
    //         .iter()
    //         .filter(|(_, _, input)| i * 5 == input.parse::<u32>().unwrap())
    //         .collect();
    //
    //     if koys.len() < 2 {
    //         println!("{:?}", koys);
    //     }
    // }
    let missing_data: Vec<(&String, &String, &String, u32)> = metrics_map
        .iter()
        .clone()
        .filter(|((_, _func_name, _), m)| m.num_invoked < 5)
        .map(|(k, m)| (&k.0, &k.1, &k.2, m.num_invoked))
        .collect();

    println!("Missing data: {:?}", missing_data);

    return;

    let keys: Vec<&(String, String, String)> = metrics_map
        .iter()
        .filter(|((_, _func_name, _), m)| m.num_invoked < 5)
        .map(|(k, _)| k)
        .collect();
    println!("so many keys: {}", keys.len());

    // let keys: Vec<_> = keys.iter().take(10).collect();

    for key in keys {
        let metrics = metrics_map.get(key).unwrap();
        println!("Function Type: {}", key.0);
        println!("Function Name: {}", key.1);
        println!("Input: {}", key.2);
        println!("Invoked: {}", metrics.num_invoked);
        println!(
            "Startup Time - Min: {:.2}ms, Max: {:.2}ms, Median: {:.2}ms, Mean: {:.2}ms, Std Deviation: {:.2}ms",
            metrics.startup_time.min / 1000.0,
            metrics.startup_time.max / 1000.0,
            metrics.startup_time.median / 1000.0,
            metrics.startup_time.mean / 1000.0,
            metrics.startup_time.std_deviation / 1000.0,

        );
        println!(
            "Total Runtime - Min: {:.2}ms, Max: {:.2}ms, Median: {:.2}ms, Mean: {:.2}ms, Std Deviation: {:.2}ms",
            metrics.runtime.min / 1000.0,
            metrics.runtime.max / 1000.0,
            metrics.runtime.median / 1000.0,
            metrics.runtime.mean / 1000.0,
            metrics.runtime.std_deviation / 1000.0
        );
        println!(
            "Average Power - Min: {:.2}W, Max: {:.2}W, Median: {:.2}W, Mean: {:.2}W, Std Deviation: {:.2}W",
            metrics.average_power.min,
            metrics.average_power.max,
            metrics.average_power.median,
            metrics.average_power.mean,
            metrics.average_power.std_deviation,

        );
        println!(
            "Average Power Isolated - Min: {:.3}W, Max: {:.3}W, Median: {:.4}W, Mean: {:.4}W, Std Deviation: {:.2}W",
            metrics.average_power_isolated.min,
            metrics.average_power_isolated.max,
            metrics.average_power_isolated.median,
            metrics.average_power_isolated.mean,
            metrics.average_power_isolated.std_deviation,

        );
        println!(
            "Energy Consumption WH - Min: {:.3}μWh, Max: {:.3}μWh, Median: {:.3}μWh, Mean: {:.3}μWh, Std Deviation: {:.3}μWh",
            metrics.energy_consumption_wh.min * 1_000_000.0,
            metrics.energy_consumption_wh.max * 1_000_000.0,
            metrics.energy_consumption_wh.median * 1_000_000.0,
            metrics.energy_consumption_wh.mean * 1_000_000.0,
            metrics.energy_consumption_wh.std_deviation * 1_000_000.0,
        );
        println!(
            "Energy Consumption Isolated WH - Min: {:.3}μWh, Max: {:.3}μWh, Median: {:.3}μWh, Mean: {:.3}μWh, Std Deviation: {:.3}μWh",
            metrics.energy_consumption_isolated_wh.min * 1_000_000.0,
            metrics.energy_consumption_isolated_wh.max * 1_000_000.0,
            metrics.energy_consumption_isolated_wh.median * 1_000_000.0,
            metrics.energy_consumption_isolated_wh.mean * 1_000_000.0,
            metrics.energy_consumption_isolated_wh.std_deviation * 1_000_000.0,
        );
        println!("---------------------------------------");
    }
}

pub fn get_missing_data(
    metrics_map: HashMap<(String, String, String), EnergyMetrics>,
) -> Vec<(String, String, String, u32)> {
    metrics_map
        .iter()
        .filter(|((_, _func_name, _), m)| m.num_invoked < 5)
        .map(|(k, m)| (k.0.clone(), k.1.clone(), k.2.clone(), m.num_invoked))
        .collect()
}
