use std::fs;

use nebula_lib::wasm_runner::serialize_wasm_modules;

pub fn serialize_modules() {
    fs::create_dir_all("./nebula/serialized").unwrap();
    let home_dir = home::home_dir().expect("Home dir not found");

    let wasm_module_dir = home_dir.join("modules/wasm");

    let serialized_tar_dir = home_dir.join(".nebula/serialized");

    let _ = serialize_wasm_modules(wasm_module_dir, serialized_tar_dir);
}
