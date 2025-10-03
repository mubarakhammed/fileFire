mod handlers;
mod models;
mod middleware;

use axum::{
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
    response::Json,
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use std::net::SocketAddr;
use std::sync::Arc;
use filefire_core::FilefireEngine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<FilefireEngine>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Initialize FileFire engine
    let engine = Arc::new(FilefireEngine::new());
    let state = AppState { engine };
    
    // Build the application with routes
    let app = create_app(state);
    
    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    log::info!("FileFire Cloud API starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

fn create_app(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Document operations
        .route("/api/v1/documents", post(handlers::upload_document))
        .route("/api/v1/documents/:id", get(handlers::get_document))
        .route("/api/v1/documents/:id", delete(handlers::delete_document))
        .route("/api/v1/documents/:id/metadata", get(handlers::get_metadata))
        
        // Document processing
        .route("/api/v1/documents/:id/convert", post(handlers::convert_document))
        .route("/api/v1/documents/:id/ocr", post(handlers::perform_ocr))
        .route("/api/v1/documents/:id/watermark", post(handlers::add_watermark))
        .route("/api/v1/documents/:id/annotations", post(handlers::add_annotation))
        .route("/api/v1/documents/:id/annotations", get(handlers::get_annotations))
        .route("/api/v1/documents/:id/annotations/:annotation_id", delete(handlers::remove_annotation))
        
        // Plugin operations
        .route("/api/v1/plugins", get(handlers::list_plugins))
        .route("/api/v1/plugins/:name/status", get(handlers::plugin_status))
        
        // Batch operations
        .route("/api/v1/batch/convert", post(handlers::batch_convert))
        .route("/api/v1/batch/ocr", post(handlers::batch_ocr))
        .route("/api/v1/batch/jobs/:id", get(handlers::get_batch_job))
        
        // Static file serving
        .nest_service("/docs", ServeDir::new("docs"))
        
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(middleware::request_logging())
        )
        .with_state(state)
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}