// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Small example of how to instantiate a wasm module that imports one function,
//! showing how you can fill in host functionality for a wasm module.

// You can execute this example with `cargo run --example hello`

use std::{fs, io, path::PathBuf};

use nebs::utilities::{hello_world::run_modules, wasm_module::FunctionResult};

#[tauri::command]
fn run_module(module_path: String) -> Result<FunctionResult, ()> {
    let result = run_modules(module_path);

    Ok(result)
}

#[tauri::command]
fn get_wats() -> Vec<PathBuf> {
    let entries = fs::read_dir("wats")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    println!("{:?}", entries);

    entries
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, run_module, get_wats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
