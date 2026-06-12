use serde_json::{Value, json};

use crate::models::{errors::AppError, types::FinancialAnalysis};


// Recibe la estructura pura que vino de Gemini y la prepara para la descarga
pub async fn generate_database(analysis: FinancialAnalysis) -> Result<Value, AppError> {

  let power_bi_metadata = json!({
    "department": analysis.department,
    "incidence_date": analysis.incidence_date,
    "deviation_type": analysis.deviation_type,
    "risk_level": analysis.risk_level,
    "risk_type": analysis.risk_type,
    "responsible_users": analysis.responsible_users,
    "user_role": analysis.user_role,
    "access_rights": analysis.access_rights,
    "data_sensitivity": analysis.data_sensitivity,
    "category": analysis.category,
    "financial_impact": analysis.financial_impact,
    "moneda": "USD",
    // "fecha_procesamiento": chrono::Utc::now().to_rfc3339() // Requiere agregar chrono al Cargo.toml o quitar esta línea
  });

  Ok(power_bi_metadata)
}
