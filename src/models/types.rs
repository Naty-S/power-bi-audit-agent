#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};


/// Structure to hold a file in memory before sending to Google
#[derive(Debug, Clone, Serialize)]
pub struct FileBuffer {
  pub name: String,
  pub mime_type: String,
  pub data: Vec<u8>,
}

/// Structure for the initial request coming from Frontend (Multipart)
#[derive(Debug, Serialize)]
pub struct AnalysisRequest {
  pub prompt: String,
  pub files: Vec<FileBuffer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResource {
  pub uri: String,
  pub mime_type: String,
}

/// Structure for the chat/correction request (JSON)
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
  pub files_resources: Vec<FileResource>,
  pub history: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
  pub role: String,
  pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ChatRole {
  USER,
  AI
}

/// Structure for the financial analysis result
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialAnalysis {  
  pub step1_audit_comparison: String,
  pub step2_fraud_detection: String,
  pub step3_access_anomalies: String,
  pub step4_payroll_inconsistencies: String,
  pub step5_financial_comparison: String,
  pub department: String,
  pub incidence_date: String, // (Format: YYYY-MM-DD HH:MM:SS).
  pub deviation_type: Deviation,
  pub risk_level: RiskLevel,
  pub risk_type: RiskType,
  pub responsible_users: Vec<String>,
  pub user_role: UserRole,
  pub access_rights: Access,
  pub data_sensitivity: DataType,
  pub category: FindingCategory,
  pub financial_impact: String,
  pub summary: String,
  pub details: String,
  pub files: Vec<FileResource>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Deviation {
  // #[msg("Pago de nómina a un empleado inactivo")]
  Payment_to_inactive_employee,
  Unregistered_supplier_payment,
  Unbilled_product_sale,
  Off_hours_inventory_movement,
  Unauthorized_debt_collection_management,
  Other_off_hours_unauthorized_payments
  // pago de nómina a un empleado inactivo, pago a un proveedor no registrado, venta de un producto no facturado, movimiento de inventario fuera de hora, 
  // gestión de cobranza no autorizada, otros pagos no autorizados fuera de hora
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum RiskLevel {
  Low, Medium, High, Critical
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum RiskType {
  Data_theft, Unauthorized_access, Financial_movements, Data_modification
  // Sustracción de datos, acceso indebido, movimientos financieros, modificación de datos.
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum UserRole {
  Analyst, Department_head, Manager, Accountant, Administrator
  // Analista, Jefe de Dpto., Gerente, Contador, Administrador
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Access {
  Read, Write, Execute, Delete
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum DataType {
  Public, Internal, Financial_transactions, Restricted
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum FindingCategory {
  Payroll, System_access, Financial_deviation
}
