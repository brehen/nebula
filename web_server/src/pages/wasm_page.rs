use std::path::Path;

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
    let modules = list_files("/Users/mariuskluften/projects/modules/wasm")
        .expect("There to be modules on the server");
    let modules: Vec<String> = modules
        .iter()
        .filter_map(|path| Path::new(path).file_stem())
        .map(|name| name.to_str().unwrap().to_string())
        .collect();
    let template = WasmTemplate { modules };
    HtmlTemplate(template)
}
