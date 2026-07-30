use axum::{
  routing::post,
  // extract::{DefaultBodyLimit},
  Router
};
use reqwest::{
  Method,
  header::{ AUTHORIZATION, ACCEPT, CONTENT_TYPE }
};
use tower_http::cors::{CorsLayer};
use std::net::SocketAddr;
use dotenv::dotenv;
use std::env;

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

  let cors = CorsLayer::new()
    .allow_origin([
      "https://shiori-web.onrender.com".parse().unwrap(),
      "http://localhost:5173".parse().unwrap(),
      "http://localhost:8080".parse().unwrap()
    ]) 
    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

  // Configurar la ruta para la API
  let app = Router::new()

    // The /analyze endpoint receives both initial upload (Multipart) 
    // and chat corrections (JSON)
    .route("/analyze", post(analyze::handler))

    // Endpoint to format the final file
    .route("/generate", post(generate::handler))
    
    // TODO: Límite de tamaño de archivo (10MB). Multiple files, how, total?
    // .layer(DefaultBodyLimit::max(10 * 1024 * 1024));
    .layer(cors); 

  // Start server
  // Render inyecta el puerto automáticamente. Si no existe (estás en local), usará el 8080.
  let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
  let addr = format!("0.0.0.0:{}", port);
    
  println!("🚀 Server at http://{}", addr);

  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
