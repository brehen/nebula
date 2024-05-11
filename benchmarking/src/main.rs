use benchmarking::{
    baseline::read_baseline,
    benchmarking::{benchmark_nebula_nrec, benchmark_nebula_rpi},
    utils::{
        efficiency_statistics::{analyze_efficiency_data, EfficiencyMetrics},
        energy_statistics::{analyze_power_data, print_analyzed_power_metrics, EnergyMetrics},
        file::{read_values, write_results},
        request::FunctionResult,
    },
};
use serde_derive::Serialize;

#[derive(Serialize)]
struct Respi {
    func_name: String,
    func_type: String,
    input: String,
    metrics: EfficiencyMetrics,
}
#[derive(Serialize)]
struct Respi2 {
    func_name: String,
    func_type: String,
    input: String,
    metrics: EnergyMetrics,
}

#[tokio::main]
async fn main() {
    let env_arg = std::env::args().nth(1).unwrap_or("".to_owned());
    if env_arg == "baseline" {
        read_baseline().await;
    } else if env_arg == "nrec" {
        benchmark_nebula_nrec().await;
    } else if env_arg == "rpi" {
        benchmark_nebula_rpi(false).await;
    } else if env_arg == "rpi_fill_gaps" {
        benchmark_nebula_rpi(true).await;
    } else if env_arg == "print_rpi" {
        let previous_readings: Vec<FunctionResult> = read_values("energy_data_rpi").await.unwrap();

        let analyzed = analyze_power_data(&previous_readings);

        print_analyzed_power_metrics(&analyzed);
    } else if env_arg == "analyze_nrec" {
        let previous_readings: Vec<FunctionResult> = read_values("energy_data_nrec").await.unwrap();

        println!("Found {} readings from the file.", previous_readings.len());

        let analyzed = analyze_efficiency_data(&previous_readings);

        let analyzed_nrec: Vec<_> = analyzed
            .into_iter()
            .map(|(k, data)| Respi {
                func_type: k.0,
                func_name: k.1,
                input: k.2,
                metrics: data,
            })
            .collect();

        let _ = write_results(&analyzed_nrec, "analyzed_nrec_data_0_4_0").await;
    } else if env_arg == "analyze_rpi" {
        let previous_readings: Vec<FunctionResult> = read_values("energy_data_rpi").await.unwrap();

        println!("Found {} readings from the file.", previous_readings.len());

        let analyzed = analyze_power_data(&previous_readings);

        let analyzed_nrec: Vec<_> = analyzed
            .into_iter()
            .map(|(k, data)| Respi2 {
                func_type: k.0,
                func_name: k.1,
                input: k.2,
                metrics: data,
            })
            .collect();

        let _ = write_results(&analyzed_nrec, "analyzed_rpi_data_0_4_0").await;
    } else {
        eprintln!(
            "Please provide arg for the desired benchmarking! Options: baseline | nrec | rpi"
        );
    }
}
