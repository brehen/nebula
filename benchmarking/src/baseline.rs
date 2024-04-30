use std::time::Duration;
use tokio::time::sleep;

use crate::utils::{
    baseline::measure_baseline,
    file::write_results,
    modbus::{read_modbus_data, SensorData},
};

pub async fn read_baseline() {
    let file_name = "baseline_readings";
    let mut sensor_readings: Vec<SensorData> = vec![];

    // Just sleep for the amount of time I want to use to measure idle baseline
    let mut get_baseline_power = tokio::spawn(async move {
        sleep(Duration::from_secs(60 * 20)).await;
    });

    loop {
        tokio::select! {
            data = read_modbus_data("192.168.68.66:502") => {
                let data = data.unwrap();
                println!("Read: {:?}", &data);
                sensor_readings.push(data);
            }
            // Thread for sleeping while doing power measurements of idle load
            _ = &mut get_baseline_power => {
                    break;
                }
        }
    }

    let _ = write_results(&sensor_readings, file_name).await;

    println!(
        "Average power while idle was: {}",
        measure_baseline(sensor_readings)
    );
}
