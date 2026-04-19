
/* 
while (archivo)
  check valid extension
  to base 64
  send to gemini
  generate power bi json
  return json

Error no files
*/
use axum::{
  extract::{FromRequest, Multipart, Request},
  Json,
  http::StatusCode,
  response::IntoResponse
};
use serde_json::json;

use crate::models::types::*;
use crate::models::errors::AppError;
use crate::services::file_management;
use crate::services::gemini;


/// Main entry point for the /analyze endpoint.
/// It acts as a dispatcher:
/// - If Content-Type is multipart -> Process form data to send to IA as an analysis request
/// - If Content-Type is json -> Process prompt to make adjustments to re-analyze
pub async fn handler(req: Request) -> Result<impl IntoResponse, AppError> {
  
  println!("-----______in handler");

  let headers = req.headers().clone();
  let content_type = headers.get("content-type")
    .and_then(|val| val.to_str().ok())
    .unwrap_or("");

  if content_type.starts_with("multipart/form-data") { // Process form data to send to IA

    let form_files = Multipart::from_request(req, &headers).await
      .map_err(|e| AppError::UploadError(e.to_string()))?;    
    let analysis_request = file_management::process_files(form_files).await?;
    let analysis = gemini::start_analysis(analysis_request).await?;

    return Ok((StatusCode::OK, Json(json!(analysis))).into_response());

  } else if content_type.starts_with("application/json") { // Use chat history and uploaded files to adjust answer

    let b = req.into_body();
    let body_bytes = axum::body::to_bytes(b, 1024 * 1024).await
      .map_err(|e| AppError::Unknown(e.to_string()))?;
    let chat_request: ChatRequest = serde_json::from_slice(&body_bytes)
      .map_err(|e| AppError::Unknown(format!("Invalid JSON format: {}", e)))?;
    let adjust = gemini::adjust_analysis(chat_request).await?;

    return Ok((StatusCode::OK, Json(json!(adjust))).into_response());
  }

  Err(AppError::UploadError("Unsupported Content-Type. Use multipart/form-data or application/json".to_string()))
}
