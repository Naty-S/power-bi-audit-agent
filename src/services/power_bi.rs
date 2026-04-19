use axum::{
  Json,
  response::IntoResponse,
  http::StatusCode
};
use serde_json::json;

use crate::models::types::FinancialAnalysis;


// Recibe la estructura pura que vino de Gemini y la prepara para la descarga
pub async fn generate_database(analysis: FinancialAnalysis) -> impl IntoResponse {

  let power_bi_metadata = json!({
    "status": "success",
    "power_bi_data": {
      "ingresos_totales": analysis.income,
      "egresos_totales": analysis.outcome,
      "moneda": "USD", // Ejemplo de campo extra para Power BI
      // "fecha_procesamiento": chrono::Utc::now().to_rfc3339() // Requiere agregar chrono al Cargo.toml o quitar esta línea
    }
  });

  (StatusCode::OK, Json(power_bi_metadata)).into_response()
}
