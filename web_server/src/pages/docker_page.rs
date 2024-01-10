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
    let images = list_files("/Users/mariuskluften/projects/modules/wasm")
        .expect("there to be modules on the server");

    let images: Vec<String> = images
        .iter()
        .filter_map(|path| Path::new(path).file_stem())
        .map(|name| name.to_str().unwrap().to_string())
        .collect();

    let template = DockerTemplate { images };
    HtmlTemplate(template)
}
