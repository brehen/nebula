use std::collections::HashMap;
use std::str::FromStr;

use serde_derive::Serialize;

use super::calc::median;
use super::calc::std_deviation;
use super::calc::update_min_max_avg;
use super::request::FunctionResult;
use super::request::ModuleType;
use super::Metrics; // Import the ModuleType enum

#[derive(Default, Debug, Serialize)]
pub struct EfficiencyMetrics {
    pub startup_time: Metrics,
    pub runtime: Metrics,
    pub num_invoked: u32,
}

pub fn analyze_efficiency_data(
    measurements: &[FunctionResult],
) -> HashMap<(String, String, String), EfficiencyMetrics> {
    let mut efficiency_metrics = HashMap::new();

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
        let metrics = efficiency_metrics
            .entry(key)
            .or_insert_with(EfficiencyMetrics::default);

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
        }
    }

    calculate_medians(&mut efficiency_metrics, measurements);

    efficiency_metrics
}

fn calculate_medians(
    efficiency_metrics: &mut HashMap<(String, String, String), EfficiencyMetrics>,
    measurements: &[FunctionResult],
) {
    for ((func_type, func_name, input), metrics) in efficiency_metrics.iter_mut() {
        let startup_times: Vec<_> = measurements
            .iter()
            .filter(|m| {
                m.func_type == ModuleType::from_str(func_type).unwrap()
                    && &m.func_name == func_name
                    && &m.input == input
            })
            .flat_map(|m| m.metrics.as_ref().map(|mm| mm.startup_time as f64))
            .collect();

        let runtimes: Vec<_> = measurements
            .iter()
            .filter(|m| {
                m.func_type == ModuleType::from_str(func_type).unwrap()
                    && &m.func_name == func_name
                    && &m.input == input
            })
            .flat_map(|m| {
                m.metrics
                    .as_ref()
                    .map(|mm| mm.total_runtime as f64 - mm.startup_time as f64)
            })
            .collect();

        metrics.startup_time.median = median(&startup_times).unwrap_or(0.0);
        metrics.runtime.median = median(&runtimes).unwrap_or(0.0);

        metrics.startup_time.mean /= metrics.num_invoked as f64;
        metrics.runtime.mean /= metrics.num_invoked as f64;

        metrics.startup_time.std_deviation = std_deviation(&startup_times).unwrap_or(0.0); // Calculate standard deviation
        metrics.runtime.std_deviation = std_deviation(&runtimes).unwrap_or(0.0);
    }
}

// pub fn draw_efficiency_metrics(
//     metrics_map: &HashMap<(String, String, String), EfficiencyMetrics>,
// ) -> anyhow::Result<()> {
//     let fibonacci_docker_metrics: Vec<(u32, EfficiencyMetrics)> = Vec::new();
//     let fibonacci_wasm_metrics: Vec<(u32, EfficiencyMetrics)> = Vec::new();
//
//     for ((func_type, func_name, input), metrics) in metrics_map {
//         if func_name == "fibonacci-recursive" {
//             if func_type == "Docker" {
//                 fibonacci_docker_metrics.push((input.parse().unwrap(), metrics.clone()));
//             } else {
//                 fibonacci_wasm_metrics.push((input.parse().unwrap(), metrics.clone()));
//             }
//         }
//     }
//
//     if !fibonacci_docker_metrics.is_empty() && !fibonacci_wasm_metrics.is_empty() {
//         let root = BitMapBackend::new("fibonacci_metrics.png", (640, 480)).into_drawing_area();
//         root.fill(&WHITE)?;
//
//         let mut chart = ChartBuilder::on(&root)
//             .caption(
//                 "Performance Metrics for fibonacci",
//                 ("sans-serif", 50).into_font(),
//             )
//             .margin(5)
//             .x_label_area_size(30)
//             .y_label_area_size(30)
//             .build_ranged(
//                 fibonacci_docker_metrics
//                     .iter()
//                     .chain(fibonacci_wasm_metrics.iter())
//                     .map(|(input, _)| *input as i32)
//                     .min()
//                     .unwrap()
//                     ..=fibonacci_docker_metrics
//                         .iter()
//                         .chain(fibonacci_wasm_metrics.iter())
//                         .map(|(input, _)| *input as i32)
//                         .max()
//                         .unwrap(),
//                 0.0..1000000.0,
//             )?;
//
//         chart
//             .configure_mesh()
//             .light_line_style(&WHITE)
//             .x_desc("Input")
//             .y_desc("Time (ns)")
//             .draw()?;
//
//         for (input, metrics) in &fibonacci_docker_metrics {
//             chart.draw_series(
//                 LineSeries::new(
//                     (0..metrics.num_invoked).map(|x| x + 1),
//                     metrics.startup_time.min..=metrics.startup_time.max,
//                 )
//                 .point_size(2)
//                 .point_fill(&BLUE)
//                 .point_stroke(&BLACK)
//                 .title(&format!("Docker Startup Time (Input: {})", input)),
//             )?;
//
//             chart.draw_series(
//                 LineSeries::new(
//                     (0..metrics.num_invoked).map(|x| x + 1),
//                     metrics.total_runtime.min..=metrics.total_runtime.max,
//                 )
//                 .point_size(2)
//                 .point_fill(&RED)
//                 .point_stroke(&BLACK)
//                 .title(&format!("Docker Total Runtime (Input: {})", input)),
//             )?;
//         }
//
//         for (input, metrics) in &fibonacci_wasm_metrics {
//             chart.draw_series(
//                 LineSeries::new(
//                     (0..metrics.num_invoked).map(|x| x + 1),
//                     metrics.startup_time.min..=metrics.startup_time.max,
//                 )
//                 .point_size(2)
//                 .point_fill(&GREEN)
//                 .point_stroke(&BLACK)
//                 .title(&format!("Wasm Startup Time (Input: {})", input)),
//             )?;
//
//             chart.draw_series(
//                 LineSeries::new(
//                     (0..metrics.num_invoked).map(|x| x + 1),
//                     metrics.total_runtime.min..=metrics.total_runtime.max,
//                 )
//                 .point_size(2)
//                 .point_fill(&YELLOW)
//                 .point_stroke(&BLACK)
//                 .title(&format!("Wasm Total Runtime (Input: {})", input)),
//             )?;
//         }
//
//         root.present()?;
//     }
//     Ok(())
// }

pub fn print_analyzed_efficiency_metrics(
    metrics_map: &HashMap<(String, String, String), EfficiencyMetrics>,
) {
    let mut keys: Vec<&(String, String, String)> = metrics_map
        .iter()
        .filter(|((_, func_name, _), _)| func_name == "fibonacci-recursive")
        .map(|(k, _)| k)
        .collect();
    println!("rpi keys: {}", keys.len());

    keys.sort_unstable_by_key(|&key| key.2.parse::<u32>().unwrap());

    // let keys: Vec<_> = keys.iter().take(10).collect();

    for key in keys {
        let metrics = metrics_map.get(key).unwrap();
        println!("{:?}", metrics);

        println!("Function Type: {}", key.0);
        println!("Function Name: {}", key.1);
        println!("Input: {}", key.2);
        println!("Invoked: {}", metrics.num_invoked);
        println!("Startup time:");
        println!(
            "Min: {:.2}ms, Max: {:.2}ms, Median: {:.2}ms, Mean: {:.2}ms, Std Deviation: {:.2}ms",
            metrics.startup_time.min / 1000.0,
            metrics.startup_time.max / 1000.0,
            metrics.startup_time.median / 1000.0,
            metrics.startup_time.mean / 1000.0,
            metrics.startup_time.std_deviation / 1000.0
        );
        println!("Total time:");
        println!(
            "Min: {:.2}ms, Max: {:.2}ms, Median: {:.2}ms, Mean: {:.2}ms, Std Deviation: {:.2}ms",
            metrics.runtime.min / 1000.0,
            metrics.runtime.max / 1000.0,
            metrics.runtime.median / 1000.0,
            metrics.runtime.mean / 1000.0,
            metrics.runtime.std_deviation / 1000.0
        );
        println!("---------------------------------------");
    }
}
