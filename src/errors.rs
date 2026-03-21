use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum AppError {
  #[error("File upload error: {0}")]
  UploadError(String),
  
  #[error("Gemini API error: {0}")]
  GeminiError(String),
  
  #[error("Server configuration error: {0}")]
  ConfigError(String),

  #[error("Unknown error: {0}")]
  Unknown(String),
}

// Implementation to convert AppError into an Axum Response
impl IntoResponse for AppError {
  fn into_response(self) -> Response {

    let (status, error_message) = match self {
      AppError::UploadError(msg) => (StatusCode::BAD_REQUEST, msg),
      AppError::GeminiError(msg) => (StatusCode::BAD_GATEWAY, msg),
      AppError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
      AppError::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
    };

    // JSON response with the error message
    let body = Json(json!({ "error": error_message }));

    (status, body).into_response()
  }
}
