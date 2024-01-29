use askama::Template;
use axum::response::IntoResponse;

use crate::utilities::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path = "pages/home.rs.html")]
pub struct HomeTemplate {}

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}
