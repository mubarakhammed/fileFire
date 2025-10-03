use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentUploadRequest {
    pub filename: String,
    pub content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentUploadResponse {
    pub id: String,
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub upload_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMetadataResponse {
    pub id: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Vec<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub page_count: u32,
    pub file_size: u64,
    pub mime_type: String,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertRequest {
    pub target_format: String,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertResponse {
    pub job_id: String,
    pub status: String,
    pub download_url: Option<String>,
    pub estimated_completion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OCRRequest {
    pub language: Option<String>,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OCRResponse {
    pub job_id: String,
    pub text: Option<String>,
    pub confidence: Option<f32>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatermarkRequest {
    pub text: String,
    pub opacity: f32,
    pub position: String,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatermarkResponse {
    pub job_id: String,
    pub status: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationRequest {
    pub page: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub content: String,
    pub annotation_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationResponse {
    pub id: String,
    pub page: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub content: String,
    pub annotation_type: String,
    pub author: Option<String>,
    pub created_at: String,
    pub modified_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest {
    pub documents: Vec<String>,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponse {
    pub job_id: String,
    pub status: String,
    pub total_documents: usize,
    pub processed_documents: usize,
    pub failed_documents: usize,
    pub estimated_completion: Option<String>,
    pub results: Vec<BatchJobResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchJobResult {
    pub document_id: String,
    pub status: String,
    pub result_url: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub capabilities: Vec<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub code: Option<String>,
    pub timestamp: String,
}
