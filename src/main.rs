mod components;
mod cv;
mod filters;
mod gradient;
mod hexgrid;
mod pages;
use anyhow::Result;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Extension, Router,
};
use tower_http::services::ServeDir;

const PORT: u32 = 3002;

#[tokio::main]
async fn main() -> Result<()> {
    let assets_dir = ServeDir::new("assets");
    let app = Router::new()
        .route(
            "/cv",
            get(pages::cv).layer(Extension(serde_qs::axum::QsQueryConfig::new(5, false))),
        )
        .route("/cv/file", get(pages::cv_data).post(pages::form_from_file))
        .route("/", get(pages::form))
        .nest_service("/assets", assets_dir)
        .fallback(|| async { Redirect::to("/") });
    let addr = tokio::net::TcpListener::bind(&format!("localhost:{PORT}")).await?;
    println!("Listening on http://localhost:{PORT}");
    axum::serve(addr, app).await?;
    Ok(())
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
