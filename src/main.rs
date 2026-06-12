use axum::{
  routing::post,
  // extract::{DefaultBodyLimit},
  Router
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use dotenv::dotenv;

// use std::path::Path;
// use tokio::fs;
// use tokio::io::AsyncWriteExt; // Write file

// use axum::{
// };

use power_bi_audit_agent::routes::*;


#[tokio::main]
async fn main() {

  dotenv().ok();

  // 1. Start Logs
  tracing_subscriber::fmt::init();

  // Configurar la ruta para el frontend y la API
  let app = Router::new()
    // Load front
    .fallback_service(ServeDir::new("frontend"))

    // The /analyze endpoint receives both initial upload (Multipart) 
    // and chat corrections (JSON)
    .route("/analyze", post(analyze::handler))

    // Endpoint to format the final file
    .route("/generate", post(generate::handler));
    
    // TODO: Límite de tamaño de archivo (10MB). Multiple files, how, total?
    // .layer(DefaultBodyLimit::max(10 * 1024 * 1024));

  // Start server
  let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
  println!("🚀 Server at http://localhost:9090");

  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
