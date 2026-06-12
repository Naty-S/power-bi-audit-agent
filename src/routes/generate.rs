use axum::{
  Json,
  http::StatusCode,
  response::IntoResponse
};

use crate::models::types::*;
use crate::models::errors::AppError;
use crate::services::power_bi;


pub async fn handler(Json(payload): Json<FinancialAnalysis>) -> Result<impl IntoResponse, AppError> {

  let database = power_bi::generate_database(payload).await?;

  Ok((StatusCode::OK, Json(database)).into_response())
}
