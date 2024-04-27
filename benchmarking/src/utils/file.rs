use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use super::modbus::SensorData;
const VERSION: &str = "0.3.1";

pub fn get_data_path(file_name: &str) -> PathBuf {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let app_dir = home_dir.join(".nebula");
    let vers_dir = app_dir.join(VERSION);

    fs::create_dir_all(&vers_dir).expect("Failed to create version directory");

    vers_dir.join(format!("{}.json", file_name))
}

pub async fn read_baseline() -> io::Result<Vec<SensorData>> {
    let baseline_path = get_data_path("baseline_readings");
    let mut file = File::open(baseline_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let results = serde_json::from_str(&contents)?;
    Ok(results)
}

pub async fn write_baseline(readings: &Vec<SensorData>) -> io::Result<()> {
    let baseline_path = get_data_path("baseline_readings");

    let serialized = serde_json::to_string(readings)?;
    fs::write(baseline_path, serialized)?;
    Ok(())
}
