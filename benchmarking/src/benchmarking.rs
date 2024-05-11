use crate::utils::{
    efficiency_statistics::{analyze_efficiency_data, print_analyzed_efficiency_metrics},
    energy_statistics::{analyze_power_data, get_missing_data, print_analyzed_power_metrics},
    file::{read_values, write_results},
    modbus::{read_modbus_data, SensorData},
    power_estimate::associate_power_measurements,
    request::{bombard_nebula, fill_in_function_gaps, FunctionResult},
};
use reqwest::Client;

pub async fn benchmark_nebula_rpi(fill_in_gaps: bool) {
    let file_name: &str = "energy_data_rpi";
    let client = Client::new();
    let mut results: Vec<FunctionResult> = vec![];
    let mut sensor_readings: Vec<SensorData> = vec![];

    let mut previous_readings: Vec<FunctionResult> = read_values(file_name).await.unwrap();

    let analyzed = analyze_power_data(&previous_readings.clone());
    let data_to_fill_in = get_missing_data(analyzed);

    let mut bombard_handle = tokio::spawn(async move {
        let result = if !fill_in_gaps {
            bombard_nebula(client, "http://192.168.68.58/api/wasm_headless")
                .await
                .unwrap()
        } else {
            println!("Filling in the gaps");
            fill_in_function_gaps(
                client,
                "http://192.168.68.58/api/wasm_headless",
                data_to_fill_in,
            )
            .await
            .unwrap()
        };
        println!(
            "Invoked in total {} functions during this benchmark",
            result.len()
        );
        result
    });

    loop {
        tokio::select! {
            result = &mut bombard_handle => {
                results.extend(result.unwrap());
                break;
            }
            data = read_modbus_data("192.168.68.53:502") => {
                sensor_readings.push(data.unwrap());
            }
        }
    }

    let processed_results = associate_power_measurements(results, &sensor_readings);

    previous_readings.extend(processed_results.clone());

    let _ = write_results(&previous_readings, file_name).await;

    let analyzed = analyze_power_data(&previous_readings);

    print_analyzed_power_metrics(&analyzed);

    let analyzed_runtime = analyze_efficiency_data(&previous_readings);
    print_analyzed_efficiency_metrics(&analyzed_runtime);
}

// Benchmark external server on Nrec
pub async fn benchmark_nebula_nrec() {
    let file_name: &str = "energy_data_nrec";

    let mut previous_readings: Vec<FunctionResult> = read_values(file_name).await.unwrap();

    // module name, max input value, steps
    let client = Client::new();
    let result = bombard_nebula(client, "http://nebula.no/api/wasm_headless")
        .await
        .unwrap();

    previous_readings.extend(result.clone());

    let _ = write_results(&previous_readings, file_name).await;
    println!("Done! Got {} results back in total", result.len(),)

    // let analyzed = analyze_efficiency_data(&previous_readings);
    //
    // print_analyzed_efficiency_metrics(&analyzed);
}
