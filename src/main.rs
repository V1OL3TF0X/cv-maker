mod components;
mod cv;
mod hexgrid;
use anyhow::Result;
use axum::{response::Redirect, routing::get, Router};
use std::os::windows::io::AsSocket;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let assets_dir = ServeDir::new("assets");
    let app = Router::new()
        .route("/", get(cv::cv_page))
        .nest_service("/assets", assets_dir)
        .fallback(|| async { Redirect::to("/") });
    let addr = tokio::net::TcpListener::bind("localhost:3002").await?;
    println!("Listening on {:?}", addr.as_socket());
    axum::serve(addr, app).await?;
    Ok(())
}
