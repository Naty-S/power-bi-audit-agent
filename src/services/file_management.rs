/* 
 * Manages reading files and send them to Gemini
 */
use std::io::{Cursor, Read};
use axum::extract::Multipart;
use mime_guess;
use calamine::{Reader, open_workbook_auto_from_rs};
use zip::ZipArchive;

use crate::models::{
  errors::AppError,
  types::{AnalysisRequest, FileBuffer}
};

/// List of allowed MIME types
const ALLOWED_MIMES: [&str; 10] = [
  "application/txt",
  "application/pdf",
  "image/png",
  "image/jpeg",
  "image/jpg",
  "text/csv",
  "application/vnd.ms-excel", // .xls
  "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", // .xlsx
  "application/msword", // .doc
  "application/vnd.openxmlformats-officedocument.wordprocessingml.document", // .docx
];


/// Processes the multipart form data, validates files, and returns the analysis start request
/// to send to the AI. 
pub async fn process_files(mut form_files: Multipart) -> Result<AnalysisRequest, AppError> {
  
  println!("-----Processing files");

  let mut files: Vec<FileBuffer> = Vec::new();
  let mut prompt = String::new();
  
  while let Some(field) = form_files.next_field().await
    .map_err(|e| AppError::UploadError(e.to_string()))?
  {
    let field_name = field.name().unwrap_or("").to_string();
    
    if field_name == "prompt" { // Extract the text prompt
      
      if let Ok(text) = field.text().await { prompt = text; }
      
    } else if field_name == "files[]" || field_name == "file" {

      let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
      println!("content_type: {}", content_type);    
      let mut filename = field.file_name().unwrap_or("unknown").to_string();
      println!("filename: {}", filename);    
      let mut data = field.bytes().await
        .map_err(|e| AppError::UploadError(format!("Failed to read file {}: {}", filename, e)))?
        .to_vec();
      println!("BYTES ok");

      if !(data.len() > 0) {
        return Err(AppError::UploadError(format!("File empty: {}", filename)));
      }
      
      if !is_allowed_mime(&content_type) && !is_allowed_extension(&filename) {
        return Err(AppError::UploadError(format!("File type not supported: {}", filename)));
      }
      println!("ALLOWED TYPE & EXT");

      // Correct the MIME type if necessary
      let mut mime_type = mime_guess::from_path(&filename).first_or_octet_stream().to_string();
      println!("mime_type: {}", mime_type);    
      // --- Convert Office files to plain text ---
      let ext = filename.split('.').last().unwrap_or("").to_lowercase();
      println!("ext: {}", ext);    
      let mut text = "No text extracted".to_string();
      
      if ext == "xls" || ext == "xlsx" { text = excel_to_text(&data)?; }
      else if ext == "docx" { text = word_to_text(&data)?; }
      else if ext == "doc" {
        return Err(AppError::UploadError("Unsuported file (.doc 97), use a most recent word document".to_string()));
      }

      if ext == "xls" || ext == "xlsx" || ext == "docx" {
        data = text.into_bytes();
        filename = format!("{}.txt", filename);
        println!("filename (is word/excel)?: {}", filename);
        mime_type = "text/plain".to_string();
      }
      // -----------------------------------------------------------      

      files.push(FileBuffer { name: filename, mime_type, data });
      println!("_________________________________________________file pushed, count: {}", files.len());
    }
  }
  
  if files.is_empty() {
    return Err(AppError::UploadError("No files were uploaded".to_string()));
  }

  println!("-------------------OK processing file----------------------");
  Ok(AnalysisRequest {prompt, files})
}


// ==========================================
// HELPER FUNCTIONS
// ==========================================


/// Helper function to validate MIME types
fn is_allowed_mime(mime: &str) -> bool {
  ALLOWED_MIMES.contains(&mime)
}

/// Helper function to validate extensions (fallback)
fn is_allowed_extension(filename: &str) -> bool {

  let lower_name = filename.to_lowercase();

  lower_name.ends_with(".txt") || 
  lower_name.ends_with(".pdf") || 
  lower_name.ends_with(".png") || 
  lower_name.ends_with(".jpg") || 
  lower_name.ends_with(".jpeg") || 
  lower_name.ends_with(".csv") ||
  lower_name.ends_with(".xls") ||
  lower_name.ends_with(".xlsx") ||
  lower_name.ends_with(".doc") ||
  lower_name.ends_with(".docx")
}

/// Extracts all text from Excel sheets and formats it as CSV-like strings
fn excel_to_text(bytes: &[u8]) -> Result<String, AppError> {
  
  let cursor = Cursor::new(bytes);
  let mut extracted_text = String::new();
  let mut workbook = open_workbook_auto_from_rs(cursor)
    .map_err(|e| AppError::UploadError(format!("Failed to open Excel: {}", e)))?;
  let sheet_names = workbook.sheet_names().to_owned();

  for sheet in sheet_names {
  
    extracted_text.push_str(&format!("\n--- Sheet: {} ---\n", sheet));
    
    if let Ok(range) = workbook.worksheet_range(&sheet) {
      for row in range.rows() {
        // Convert each cell to string and join with commas
        let row_str: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
  
        extracted_text.push_str(&row_str.join(", "));
        extracted_text.push('\n');
      }
    }
  }
  
  Ok(extracted_text)
}

/// Extracts raw text from a DOCX file by unzipping and stripping XML tags
fn word_to_text(bytes: &[u8]) -> Result<String, AppError> {
  
  let cursor = Cursor::new(bytes);
  let mut xml_data = String::new();
  let mut archive = ZipArchive::new(cursor)
    .map_err(|e| AppError::UploadError(format!("Failed to open DOCX: {}", e)))?;
  let mut file = archive.by_name("word/document.xml") // main text
    .map_err(|_| AppError::UploadError("Invalid DOCX format: word/document.xml not found".to_string()))?;
  
  file.read_to_string(&mut xml_data)
    .map_err(|e| AppError::UploadError(format!("Failed to read DOCX XML: {}", e)))?;
  
  // XML tag stripper to get clean text
  let mut clean_text = String::new();
  let mut is_inside_tag = false;
  
  for c in xml_data.chars() {
    
    if c == '>' {
      is_inside_tag = false;
      clean_text.push(' '); // Add space to separate words when tags end
    }
    else if c == '<' { is_inside_tag = true; }
    else if !is_inside_tag { clean_text.push(c); }
  }
  
  // Clean up multiple spaces
  Ok(clean_text.replace("  ", " ").trim().to_string())
}
