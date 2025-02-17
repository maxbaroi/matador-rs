use crate::services::makersuite::{self, MakerSuite, MakerSuiteEndpoints};
use crate::Result;

use axum::extract::{DefaultBodyLimit, Path};

use axum::Json;
use axum::{routing::post, Extension, Router};

use std::sync::Arc;

use tracing::info;

#[derive(Clone)]
pub struct AppState {
    client: MakerSuite,
}

impl AppState {
    fn new() -> Self {
        info!("Creating new AppState");
        AppState {
            client: MakerSuite::new(),
        }
    }
}

pub fn routes() -> Router {
    info!("Setting up routes");
    let app_state = Arc::new(AppState::new());
    Router::new()
        .route(
            MakerSuiteEndpoints::GenerateText.path(),
            post(generate_text),
        )
        .route(MakerSuiteEndpoints::EmbedText.path(), post(embed_text))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .layer(Extension(app_state))
}

#[axum::debug_handler]
pub async fn generate_text(
    app_state: Extension<Arc<AppState>>,
    Path(model_id): Path<String>,
    Json(req): Json<makersuite::types::GenerateTextRequest>,
) -> Result<Json<makersuite::types::GenerateTextResponse>> {
    println!("Calling route: generate_text");
    let text_response = app_state
        .client
        .generate_text(model_id.as_str(), req)
        .await?;

    Ok(Json(text_response))
}

#[axum::debug_handler]
pub async fn embed_text(
    app_state: Extension<Arc<AppState>>,
    Json(req): Json<makersuite::types::EmbedTextRequest>,
) -> Result<Json<makersuite::types::EmbedTextResponse>> {
    info!("Calling route: embed_text");
    let embed_response = app_state.client.embed_text(req).await?;

    Ok(Json(embed_response))
}
