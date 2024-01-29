use dirs;
use std::{
    fs::{self, File},
    io::{self, Read, Result},
    path::PathBuf,
};

use nebula_lib::models::FunctionResult;

fn get_data_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let app_dir = home_dir.join(".nebula");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("Failed to create app directory");
    }

    app_dir.join("data.json")
}

pub fn save_results(results: Vec<FunctionResult>) -> io::Result<()> {
    let serialized = serde_json::to_string(&results)?;
    let file_path = get_data_path();
    fs::write(file_path, serialized)?;
    Ok(())
}

pub fn load_results() -> Result<Vec<FunctionResult>> {
    let mut file = File::open(get_data_path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let results = serde_json::from_str(&contents)?;
    Ok(results)
}
