use std::path::Path;

use askama::Template;
use axum::response::IntoResponse;
use nebula_lib::list_files::list_files;

use crate::utilities::html_template::HtmlTemplate;
#[derive(Template)]
#[template(path = "pages/docker.rs.html")]
pub struct DockerTemplate {
    pub images: Vec<String>,
}

pub async fn docker() -> impl IntoResponse {
    let home_dir = home::home_dir().expect("Home dir not found");

    let wasm_module_dir = home_dir.join("modules/wasm");

    let images =
        list_files(wasm_module_dir.to_str().unwrap()).expect("there to be modules on the server");

    let images: Vec<String> = images
        .iter()
        .filter_map(|path| Path::new(path).file_stem())
        .map(|name| name.to_str().unwrap().to_string())
        .collect();

    let template = DockerTemplate { images };
    HtmlTemplate(template)
}
