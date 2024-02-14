use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::HashMap,
    sync::Arc,
};
use tracing::info;

use crate::{models::AppState, utilities::html_template::HtmlTemplate};
use askama::Template;
use axum::{extract::State, response::IntoResponse};
use nebula_lib::models::{FunctionResult, ModuleType};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MetricData {
    name: String,
    value: u32,
}

#[derive(Template)]
#[template(path = "pages/metrics.rs.html")]
pub struct MetricsTemplate {
    pub metrics: String,
    pub metrics_grouped_by_input: String,
    pub metrics_grouped_by_module: String,
    pub input_options: Vec<u128>,
    pub module_options: Vec<String>,
}

pub async fn metrics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lock = state.function_calls.lock().await;
    let function_results: Vec<FunctionResult> = lock.clone().into_iter().rev().collect();
    let metricified = metricify_function_results(function_results.clone());

    let grouped_by_input = group_by_input_value(function_results.clone());
    let grouped_by_module = grouped_by_module(function_results);
    let input_options: Vec<String> = grouped_by_input.keys().cloned().collect();
    let module_options: Vec<String> = grouped_by_module.keys().cloned().collect();
    let sorted_options: Vec<u128> = input_options
        .iter()
        .map(|x| x.parse::<u128>().unwrap())
        .sorted()
        .collect();

    let template = MetricsTemplate {
        metrics: serde_json::to_string(&metricified).unwrap(),
        metrics_grouped_by_input: serde_json::to_string(&grouped_by_input).unwrap(),
        metrics_grouped_by_module: serde_json::to_string(&grouped_by_module).unwrap(),
        input_options: sorted_options,
        module_options,
    };

    HtmlTemplate(template)
}

#[derive(Serialize, Debug, Clone, Default)]
struct Aggregated {
    avg_startup_time: f64,
    avg_runtime: f64,
    avg_total_runtime: f64,
}

#[derive(Serialize, Debug, Clone, Default)]
struct NestedAggregated {
    docker: Aggregated,
    wasm: Aggregated,
}

fn group_by_input_value(
    func_results: Vec<FunctionResult>,
) -> HashMap<String, HashMap<String, NestedAggregated>> {
    // Assuming `input_value` is part of FunctionResult now
    let mut aggregation: HashMap<(String, String, String), (u128, u128, u128, u32)> =
        HashMap::new();

    for result in func_results.into_iter() {
        let module_type = match result.func_type {
            ModuleType::Docker => "docker",
            ModuleType::Wasm => "wasm",
        }
        .to_string();

        let key = (result.func_name, module_type, result.input);
        let entry = aggregation.entry(key).or_insert_with(|| (0, 0, 0, 0));
        if let Some(metrics) = result.metrics {
            entry.0 += metrics.startup_time;
            entry.1 += metrics.total_runtime - metrics.startup_time;
            entry.2 += metrics.total_runtime;
            entry.3 += 1;
        }
    }

    let mut result: HashMap<String, HashMap<String, NestedAggregated>> = HashMap::new();

    for (
        (func_name, module_type, input_value),
        (sum_startup, sum_runtime, sum_total_runtime, count),
    ) in aggregation
    {
        let avg_result = Aggregated {
            avg_startup_time: sum_startup as f64 / count as f64,
            avg_runtime: sum_runtime as f64 / count as f64,
            avg_total_runtime: sum_total_runtime as f64 / count as f64,
        };

        let nested_aggregated = result
            .entry(input_value)
            .or_default()
            .entry(func_name)
            .or_default();

        match module_type.as_str() {
            "wasm" => nested_aggregated.wasm = avg_result,
            "docker" => nested_aggregated.docker = avg_result,
            _ => {}
        }
    }

    result
}

// sum, min, max, count
type TimeStats = (u128, u128, u128, u32);
#[derive(Serialize, Debug, Clone, Default)]
struct AggregatedModuleStats {
    startup: [f64; 3],
    runtime: [f64; 3],
    total_time: [f64; 3],
}

#[derive(Serialize, Debug, Clone, Default)]
struct NestedAggregatedModuleStats {
    wasm: AggregatedModuleStats,
    docker: AggregatedModuleStats,
}

fn grouped_by_module(
    func_results: Vec<FunctionResult>,
) -> HashMap<String, HashMap<String, HashMap<String, AggregatedModuleStats>>> {
    // startup, runtime, total, min, max, avg
    let mut aggregation: HashMap<(String, String, String), (TimeStats, TimeStats, TimeStats)> =
        HashMap::new();

    for result in func_results.into_iter() {
        let module_type = match result.func_type {
            ModuleType::Docker => "docker",
            ModuleType::Wasm => "wasm",
        }
        .to_string();

        let key = (result.func_name, module_type, result.input);
        let entry = aggregation.entry(key).or_insert_with(|| {
            (
                (0, 1_000_000, 0, 0),
                (0, 1_000_000, 0, 0),
                (0, 1_000_000, 0, 0),
            )
        });
        // println!("{:?}: {:?}", key, entry);
        if let Some(metrics) = result.metrics {
            entry.0 = (
                metrics.startup_time + entry.0 .0,
                min(entry.0 .1, metrics.startup_time),
                max(entry.0 .2, metrics.startup_time),
                entry.0 .3 + 1,
            );
            let runtime = metrics.total_runtime - metrics.startup_time;
            entry.1 = (
                runtime + entry.1 .0,
                min(entry.1 .1, runtime),
                max(entry.1 .2, runtime),
                entry.1 .3 + 1,
            );
            entry.2 = (
                metrics.total_runtime + entry.2 .0,
                min(entry.2 .1, metrics.total_runtime),
                max(entry.2 .2, metrics.total_runtime),
                entry.2 .3 + 1,
            );
        }
    }

    let mut result: HashMap<String, HashMap<String, HashMap<String, AggregatedModuleStats>>> =
        HashMap::new();

    for ((func_name, module_type, input_value), (startup, runtime, total_time)) in aggregation {
        // println!("{:?}", startup);
        let startup_stats: [f64; 3] = [
            startup.1 as f64,
            startup.2 as f64,
            (startup.0 as f64 / startup.3 as f64).round(),
        ];

        let runtime_stats: [f64; 3] = [
            runtime.1 as f64,
            runtime.2 as f64,
            (runtime.0 as f64 / runtime.3 as f64).round(),
        ];

        let total_time_stats: [f64; 3] = [
            total_time.1 as f64,
            total_time.2 as f64,
            (total_time.0 as f64 / total_time.3 as f64).round(),
        ];

        let stats = AggregatedModuleStats {
            startup: startup_stats,
            runtime: runtime_stats,
            total_time: total_time_stats,
        };

        result
            .entry(func_name)
            .or_default()
            .entry(module_type)
            .or_default()
            .entry(input_value)
            .or_insert_with(|| stats);
    }

    println!("{:?}", result);

    result
}
fn metricify_function_results(
    func_results: Vec<FunctionResult>,
) -> HashMap<String, NestedAggregated> {
    let mut aggregation: HashMap<(String, String), (u128, u128, u128, u32)> = HashMap::new();

    for result in func_results {
        let module_type = if matches!(result.func_type, ModuleType::Docker) {
            "docker".to_owned()
        } else {
            "wasm".to_owned()
        };

        let key = (result.func_name, module_type);
        let entry = aggregation.entry(key).or_insert((0, 0, 0, 0));
        let metrics = result.metrics.unwrap();

        entry.0 += metrics.startup_time;
        entry.1 += metrics.total_runtime - metrics.startup_time;
        entry.2 += metrics.total_runtime;
        entry.3 += 1;
    }

    let mut nested_result: HashMap<String, HashMap<String, Aggregated>> = HashMap::new();

    for ((func_name, func_type), (sum_startup, sum_runtime, sum_total_runtime, count)) in
        aggregation
    {
        let avg_result = Aggregated {
            avg_startup_time: (sum_startup as f64 / count as f64) / 1_000.0,
            avg_runtime: (sum_runtime as f64 / count as f64) / 1_000.0,
            avg_total_runtime: (sum_total_runtime as f64 / count as f64) / 1_000.0,
        };
        nested_result
            .entry(func_name)
            .or_default()
            .insert(func_type, avg_result);
    }

    nested_result
        .into_iter()
        .map(|(func_name, types)| {
            let docker = types.get("docker").cloned().unwrap_or_default();
            let wasm = types.get("wasm").cloned().unwrap_or_default();
            (func_name, NestedAggregated { docker, wasm })
        })
        .collect()
}
