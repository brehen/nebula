// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nebula_lib::{list_files::list_files, wasm_runner::run_wasi_module};

#[tauri::command]
fn list_files_in_dir() -> Vec<String> {
    match list_files("wats") {
        Ok(files) => files,
        Err(e) => {
            println!("Error reading directory: {}", e);
            vec![]
        }
    }
}

#[tauri::command]
fn run_module(path: &str) {
    let result = run_wasi_module(path);
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_files_in_dir, run_module])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
