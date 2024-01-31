use askama::Template;
use axum::response::IntoResponse;

use crate::utilities::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path = "pages/about.rs.html")]
pub struct HomeTemplate {}

pub async fn about() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}
