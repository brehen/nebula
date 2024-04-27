use benchmarking::utils::{file::get_data_path, modbus::read_modbus_data, request::bombard_nebula};
use reqwest::Client;
#[allow(unused_imports)]
use std::process;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let bombard_handle = tokio::spawn(async move {
        bombard_nebula(client).await;
    });

    let read_energy_handle = tokio::spawn(async move {
        loop {
            let sensor_data = read_modbus_data("192.168.68.66:502").await.unwrap();
            println!("Read modbus data: {:?}", sensor_data);
        }
    });
    let _ = tokio::join!(bombard_handle, read_energy_handle);
    println!("After async");
}
