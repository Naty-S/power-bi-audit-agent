
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ChatRole {
  USER,
  AI
}

/// Structure for the financial analysis result
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialAnalysis {
  pub income: String,
  pub outcome: String,
  pub currency: String,
  pub ai_msg: String,
  pub files: Vec<FileResource>
}
