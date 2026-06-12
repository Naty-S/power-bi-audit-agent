use reqwest::{Client, multipart::{Part, Form}};
use serde::Deserialize;
use serde_json::{json, from_value, Value};
use std::env;

use crate::utils::constants::*;
use crate::models::{
  errors::AppError,
  types::*,
};


// ==========================================
// Analysis functions
// ==========================================

/// 
pub async fn start_analysis(analysis_req: AnalysisRequest) -> Result<FinancialAnalysis, AppError> {
    
  let api_key = env::var("GEMINI_API_KEY")
    .map_err(|_| AppError::ConfigError("GEMINI_API_KEY not found".to_string()))?;
  
  // Upload files
  let mut files: Vec<FileResource> = Vec::new();
  
  for f in analysis_req.files {
    let uri = upload_file_to_google(&api_key, &f).await?;
    files.push(FileResource { uri, mime_type: f.mime_type.clone() });
  }

  // Generate analysis
  let res = ask_gemini(&api_key, &files, &analysis_req.prompt, vec![]).await?;

  Ok(FinancialAnalysis {
    step1_audit_comparison: res["step1_audit_comparison"].to_string(),
    step2_fraud_detection: res["step2_fraud_detection"].to_string(),
    step3_access_anomalies: res["step3_access_anomalies"].to_string(),
    step4_payroll_inconsistencies: res["step4_payroll_inconsistencies"].to_string(),
    step5_financial_comparison: res["step5_financial_comparison"].to_string(),
    department: res["department"].to_string(),
    incidence_date: res["incidence_date"].to_string(),
    deviation_type: from_value(res["deviation_type"].clone()).expect("Format from deviation_type doesn't match enum Deviation"),
    risk_level: from_value(res["risk_level"].clone()).expect("Format from risk_level doesn't match enum RiskLevel"),
    risk_type: from_value(res["risk_type"].clone()).expect("Format from risk_type doesn't match enum RiskType"),
    responsible_users: from_value::<Vec<String>>(res["responsible_users"].clone()).unwrap_or_else(|_| vec![]),
    user_role: from_value(res["user_role"].clone()).expect("Format from user_role doesn't match enum UserRole"),
    access_rights: from_value(res["access_rights"].clone()).expect("Format from access_rights doesn't match enum Access"),
    data_sensitivity: from_value(res["data_sensitivity"].clone()).expect("Format from data_sensitivity doesn't match enum DataType"),
    category: from_value(res["category"].clone()).expect("Format from category doesn't match enum FindingCategory"),
    financial_impact: res["financial_impact"].to_string(),
    summary: res["summary"].to_string(),
    details: res["details"].to_string(),
    files
  })
}

/// 
pub async fn adjust_analysis(req: ChatRequest) -> Result<FinancialAnalysis, AppError> {
  
  let api_key = env::var("GEMINI_API_KEY")
    .map_err(|_| AppError::ConfigError("GEMINI_API_KEY not found".to_string()))?;

  // Extract the last message from history as the "current prompt" and keep the rest as context
  let (history, last_prompt) = if let Some(last) = req.history.last() {
    (req.history[..req.history.len()-1].to_vec(), last.text.clone())
  } else {
    return Err(AppError::Unknown("Last prompt not found".to_string()))
  };

  // Ask Gemini
  let user_prompt = "Based on the user's feedback and the document history.".to_owned() + &last_prompt;
  let res = ask_gemini(&api_key, &req.files_resources, &user_prompt, history).await?;

  Ok(FinancialAnalysis {
    step1_audit_comparison: String::deserialize(&res["step1_audit_comparison"]).unwrap_or_default(),
    step2_fraud_detection: String::deserialize(&res["step2_fraud_detection"]).unwrap_or_default(),
    step3_access_anomalies: String::deserialize(&res["step3_access_anomalies"]).unwrap_or_default(),
    step4_payroll_inconsistencies: String::deserialize(&res["step4_payroll_inconsistencies"]).unwrap_or_default(),
    step5_financial_comparison: String::deserialize(&res["step5_financial_comparison"]).unwrap_or_default(),
    department: String::deserialize(&res["department"]).unwrap_or_default(),
    incidence_date: String::deserialize(&res["incidence_date"]).unwrap_or_default(),
    deviation_type: from_value(res["deviation_type"].clone()).expect("Format from deviation_type doesn't match enum Deviation"),
    risk_level: from_value(res["risk_level"].clone()).expect("Format from risk_level doesn't match enum RiskLevel"),
    risk_type: from_value(res["risk_type"].clone()).expect("Format from risk_type doesn't match enum RiskType"),
    responsible_users: from_value::<Vec<String>>(res["responsible_users"].clone()).unwrap_or_else(|_| vec![]),
    user_role: from_value(res["user_role"].clone()).expect("Format from user_role doesn't match enum UserRole"),
    access_rights: from_value(res["access_rights"].clone()).expect("Format from access_rights doesn't match enum Access"),
    data_sensitivity: from_value(res["data_sensitivity"].clone()).expect("Format from data_sensitivity doesn't match enum DataType"),
    category: from_value(res["category"].clone()).expect("Format from category doesn't match enum FindingCategory"),
    financial_impact: String::deserialize(&res["financial_impact"]).unwrap_or_default(),
    summary: String::deserialize(&res["summary"]).unwrap_or_default(),
    details: String::deserialize(&res["details"]).unwrap_or_default(),
    files: req.files_resources
  })
}


// ==========================================
// HELPER FUNCTIONS (Google API)
// ==========================================


/// Uploads a file to Google's `upload` endpoint using Multipart.
/// Returns the `fileUri` (e.g., "https://generativelanguage.googleapis.com/v1beta/files/...")
async fn upload_file_to_google(api_key: &str, file: &FileBuffer) -> Result<String, AppError> {
    
  let client = Client::new();
  let url = format!("{}upload/v1beta/files?key={}", BASE_API_URL, api_key);
  // curl -X DELETE https://generativelanguage.googleapis.com/v1beta/files/NOMBRE?key=

  // Upload file
  // TODO: Buscar si ya está y actualizar comparando contenido

  let metadata = Part::text(json!({ "file": { "display_name": file.name } }).to_string()); // Alguna otra cosa que deba ir en la metadata?
  let file_part = Part::bytes(file.data.clone()).file_name(file.name.clone());
  let form = Form::new()
    .part("metadata", metadata.mime_str("application/json").unwrap())
    .part("file", file_part.mime_str(&file.mime_type).unwrap());

  let res = client.post(&url).header("X-Goog-Upload-Protocol", "multipart")
    .multipart(form).send().await
    .map_err(|e| AppError::GeminiError(format!("Failed to connect to upload API: {}", e)))?;

  if !res.status().is_success() {
    let err = res.text().await.unwrap_or_default();
    return Err(AppError::GeminiError(format!("Google Upload failed: {}", err)));
  }

  // Parse URI
  let body: Value = res.json().await
    .map_err(|e| AppError::GeminiError(format!("Failed to parse upload response: {}", e)))?;
  println!("{:?}", body);
  let uri = body["file"]["uri"]
    .as_str()
    .ok_or(AppError::GeminiError("Google API did not return a file URI".to_string()))?
    .to_string();

  println!("{:?}", uri);
  Ok(uri)
}

/// Calls Goggle's `generateContent` endpoint with the .
/// It constructs the payload including the File URI and Chat History.
async fn ask_gemini(
  api_key: &str, 
  files: &Vec<FileResource>, 
  user_prompt: &str,
  history: Vec<ChatMessage>
) -> Result<Value, AppError> {
    
  let client = Client::new();
  let url = format!("{}v1beta/models/gemini-2.5-flash:generateContent?key={}", BASE_API_URL, api_key);

  // 1. Convert internal ChatMessage history to Gemini's "contents" format
  let mut contents_json = Vec::new();

  // Join text + all file URIs to a message
  let attach_files = |text: &str| -> Vec<Value> {
    
    let mut parts = vec![json!({ "text": text })];
    
    for r in files {
      parts.push(json!({ "fileData": { "mimeType": r.mime_type, "fileUri": r.uri } }));
    }
    
    parts
  };
  
  // Reconstruct history for the API

  if history.is_empty() { // Initial request: User [Text + Files]
    contents_json.push(json!({ "role": "user", "parts": attach_files(user_prompt) }));
    
  } else { // Replay history

    // First message MUST have the files. WHY?? -> Assumption?
    // let first_msg = &history[0];
    contents_json.push(json!({ "role": "user", "parts": attach_files(&(&history[0]).text) }));

    // Middle messages
    for msg in history.iter().skip(1) {
      contents_json.push(json!({ "role": msg.role, "parts": [{ "text": msg.text }] }));
    }

    // Current (last) message from user
    contents_json.push(json!({ "role": "user", "parts": [{ "text": user_prompt }] }));
  }

  // 2. Send prompt to Gemini
  let body = json!({
    "contents": contents_json,
    "systemInstruction": { "parts": [{ "text": BASE_PROMPT }] },
    "generationConfig": { "responseMimeType": "application/json" }
  });
  // println!("body: {}", body);

  let res = client.post(&url).json(&body)
    .send().await
    .map_err(|e| AppError::GeminiError(format!("Gemini connection error: {}", e)))?;

  if !res.status().is_success() {
    let err = res.text().await.unwrap_or_default();
    return Err(AppError::GeminiError(format!("Gemini API Error: {}", err)));
  }

  // 3. Extract Text and Parse JSON
  let res_body: Value = res.json().await
    .map_err(|_| AppError::GeminiError("Invalid JSON from Gemini".to_string()))?;
  // println!("response from gemini: {:?}", response_body);
  
  let answer_text = res_body["candidates"][0]["content"]["parts"][0]["text"].as_str()
    .ok_or(AppError::GeminiError("No text found in Gemini response".to_string()))?;

  let answer_json: Value = serde_json::from_str(answer_text)
    .map_err(|e| AppError::GeminiError(format!("Gemini did not return valid JSON inside text: {}", e)))?;

  Ok(answer_json)
}

/* Error: Error: Gemini API Error: {
  "error": {
    "code": 429,
    "message": "You exceeded your current quota, please check your plan and billing details. For more information on this error, head to: https://ai.google.dev/gemini-api/docs/rate-limits. To monitor your current usage, head to: https://ai.dev/rate-limit. \n* Quota exceeded for metric: generativelanguage.googleapis.com/generate_content_free_tier_input_token_count, limit: 0, model: gemini-2.0-flash\n* Quota exceeded for metric: generativelanguage.googleapis.com/generate_content_free_tier_requests, limit: 0, model: gemini-2.0-flash\n* Quota exceeded for metric: generativelanguage.googleapis.com/generate_content_free_tier_requests, limit: 0, model: gemini-2.0-flash\nPlease retry in 39.915086254s.",
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.Help",
        "links": [
          {
            "description": "Learn more about Gemini API quotas",
            "url": "https://ai.google.dev/gemini-api/docs/rate-limits"
          }
        ]
      },
      {
        "@type": "type.googleapis.com/google.rpc.QuotaFailure",
        "violations": [
          {
            "quotaMetric": "generativelanguage.googleapis.com/generate_content_free_tier_input_token_count",
            "quotaId": "GenerateContentInputTokensPerModelPerMinute-FreeTier",
            "quotaDimensions": {
              "location": "global",
              "model": "gemini-2.0-flash"
            }
          },
          {
            "quotaMetric": "generativelanguage.googleapis.com/generate_content_free_tier_requests",
            "quotaId": "GenerateRequestsPerMinutePerProjectPerModel-FreeTier",
            "quotaDimensions": {
              "location": "global",
              "model": "gemini-2.0-flash"
            }
          },
          {
            "quotaMetric": "generativelanguage.googleapis.com/generate_content_free_tier_requests",
            "quotaId": "GenerateRequestsPerDayPerProjectPerModel-FreeTier",
            "quotaDimensions": {
              "location": "global",
              "model": "gemini-2.0-flash"
            }
          }
        ]
      },
      {
        "@type": "type.googleapis.com/google.rpc.RetryInfo",
        "retryDelay": "39s"
      }
    ]
  }
}

*/


/* Error: Error: Gemini API Error: {
  "error": {
    "code": 404,
    "message": "models/gemini-3.0-flash is not found for API version v1beta, or is not supported for generateContent. Call ListModels to see the list of available models and their supported methods.",
    "status": "NOT_FOUND"
  }
}
*/

/* Error: Error: Gemini API Error: { ----2.5-flash
  "error": {
    "code": 400,
    "message": "Request contains an invalid argument.",
    "status": "INVALID_ARGUMENT"
  }
}*/
