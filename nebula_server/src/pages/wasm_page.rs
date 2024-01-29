use std::{path::Path, process::Command};

use askama::Template;
use axum::response::IntoResponse;
use nebula_lib::list_files::list_files;

use crate::utilities::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path = "pages/wasm.rs.html")]
pub struct WasmTemplate {
    pub modules: Vec<String>,
}

pub async fn wasm() -> impl IntoResponse {
    let home_dir = home::home_dir().expect("Home dir not found");

    let wasm_module_dir = home_dir.join("modules/wasm");

    let modules =
        list_files(wasm_module_dir.to_str().unwrap()).expect("There to be modules on the server");
    let modules: Vec<String> = modules
        .iter()
        .filter_map(|path| Path::new(path).file_stem())
        .map(|name| name.to_str().unwrap().to_string())
        .collect();
    let template = WasmTemplate { modules };
    HtmlTemplate(template)
}
