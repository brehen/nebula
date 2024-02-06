use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::{
    api::call_function::get_fc_list, models::AppState, utilities::html_template::HtmlTemplate,
};

pub async fn get_function_results(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lock = state.function_calls.lock().await;
    let template = get_fc_list(lock.clone().into_iter().rev().collect());

    HtmlTemplate(template)
}
