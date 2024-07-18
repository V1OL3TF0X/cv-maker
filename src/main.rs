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
        .route("/cv/file", get(pages::cv_data).post(pages::form_from_file))
        .route(
            "/cv",
            get(pages::cv).layer(Extension(serde_qs::axum::QsQueryConfig::new(5, false))),
        )
        .route("/", get(pages::form))
        .nest_service("/assets", assets_dir)
        .fallback(|| async { Redirect::to("/") });
    let addr = tokio::net::TcpListener::bind(&format!("0.0.0.0:{PORT}")).await?;
    println!("Listening on http://localhost:{PORT}");
    axum::serve(addr, app).await?;
    Ok(())
}

pub struct AppError(pub StatusCode, anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, format!("Something went wrong: {}", self.1)).into_response()
    }
}

trait WithCode<O>: Sized {
    fn with_code(self, code: StatusCode) -> Result<O, AppError>;
}

impl<O, E> WithCode<O> for Result<O, E>
where
    E: Into<anyhow::Error>,
{
    fn with_code(self, code: StatusCode) -> Result<O, AppError> {
        self.map_err(|e| AppError(code, e.into()))
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::UNPROCESSABLE_ENTITY, err.into())
    }
}
