use benchmarking::{
    baseline::read_baseline,
    benchmarking::{benchmark_nebula_nrec, benchmark_nebula_rpi},
    utils::{
        efficiency_statistics::{analyze_efficiency_data, print_analyzed_efficiency_metrics},
        energy_statistics::{analyze_power_data, print_analyzed_power_metrics},
        file::read_values,
        request::FunctionResult,
    },
};

#[tokio::main]
async fn main() {
    let env_arg = std::env::args().nth(1).unwrap_or("".to_owned());
    if env_arg == "baseline" {
        read_baseline().await;
    } else if env_arg == "nrec" {
        benchmark_nebula_nrec().await;
    } else if env_arg == "rpi" {
        benchmark_nebula_rpi().await;
    } else if env_arg == "print_rpi" {
        let previous_readings: Vec<FunctionResult> = read_values("energy_data_rpi").await.unwrap();

        let analyzed = analyze_power_data(&previous_readings);

        print_analyzed_power_metrics(&analyzed);
    } else if env_arg == "print_nrec" {
        let previous_readings: Vec<FunctionResult> = read_values("energy_data_nrec").await.unwrap();

        println!("Found {} readings from the file.", previous_readings.len());

        let analyzed = analyze_efficiency_data(&previous_readings);

        println!(
            "Ended up with {} records with energy readings from the results.",
            analyzed.len()
        );

        print_analyzed_efficiency_metrics(&analyzed);
    } else {
        eprintln!(
            "Please provide arg for the desired benchmarking! Options: baseline | nrec | rpi"
        );
    }
}
