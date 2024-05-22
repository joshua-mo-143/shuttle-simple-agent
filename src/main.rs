use std::env::set_var;

use agents::MyAgent;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

pub mod agents;

use shuttle_runtime::SecretStore;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
pub struct Prompt {
    prompt: String,
}

#[derive(Clone)]
pub struct AppState {
    agent: MyAgent,
}

async fn prompt(State(state): State<AppState>, Json(json): Json<Prompt>) -> impl IntoResponse {
    match state.agent.prompt(&json.prompt).await {
        Ok(res) => (StatusCode::OK, res),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {e}"),
        ),
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    secrets.into_iter().for_each(|x| {
        set_var(x.0, x.1);
    });

    let state = AppState {
        agent: MyAgent::new(),
    };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/prompt", post(prompt))
        .with_state(state);

    Ok(router.into())
}
