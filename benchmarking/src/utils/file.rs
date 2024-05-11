use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use super::modbus::SensorData;
const VERSION: &str = "0.5.0";

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

pub async fn read_values<T>(file_name: &str) -> io::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let fr_path = get_data_path(file_name);

    println!("Reading from: {:?}", fr_path);

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // This will create the file if it doesn't exist
        .open(fr_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        contents = "[]".to_string();
    }

    let values = serde_json::from_str(&contents)?;

    Ok(values)
}

pub async fn write_results<T>(function_results: &T, file_name: &str) -> io::Result<()>
where
    T: serde::Serialize,
{
    let fr_path = get_data_path(file_name);

    let serialized = serde_json::to_string(function_results)?;
    fs::write(fr_path, serialized)?;
    Ok(())
}
