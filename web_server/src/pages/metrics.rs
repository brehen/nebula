use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use nebula_lib::models::FunctionResult;
use serde::Serialize;

use crate::{components::function_results::AppState, utilities::html_template::HtmlTemplate};

#[derive(Serialize, Debug)]
pub struct MetricData {
    name: String,
    value: u32,
}

#[derive(Template)]
#[template(path = "pages/metrics.rs.html")]
pub struct MetricsTemplate {
    pub name: String,
    pub metrics: String,
}

pub async fn metrics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lock = state.function_calls.lock().await;
    let function_results: Vec<FunctionResult> = lock.clone().into_iter().rev().collect();
    println!("{:?}", state);

    let template = MetricsTemplate {
        name: "Hey there".to_string(),
        metrics: serde_json::to_string(&function_results).unwrap(),
    };
    HtmlTemplate(template)
}
