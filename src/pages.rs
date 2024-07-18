use crate::{
    components::{Contact, TimeRange},
    cv::CV,
    AppError, WithCode,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::Multipart,
    http::{
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
        StatusCode,
    },
    response::{Html, Response},
};
use serde_qs::axum::QsQuery;
use serde_yaml::from_str;

pub async fn cv(QsQuery(cv): QsQuery<CV>) -> impl IntoResponse {
    Html(cv.render().unwrap())
}
pub async fn cv_data(QsQuery(cv): QsQuery<CV>) -> Response<Body> {
    let body = serde_yaml::to_string(&cv)
        .expect("couldn't serialize CV data to TOML.")
        .into();
    Response::builder()
        .header(CONTENT_TYPE, "text/yaml; charset=utf-8")
        .header(CONTENT_DISPOSITION, "attachment; filename=\"CV.yaml\"")
        .body(body)
        .unwrap()
}

#[derive(Template)]
#[template(path = "form.html", whitespace = "suppress")]
struct Form {
    init: CV,
}

#[derive(Template)]
#[template(path = "form_page.html", whitespace = "suppress")]
struct FormPage;

pub async fn form() -> impl IntoResponse {
    Html(FormPage.render().unwrap())
}

pub async fn form_from_file(mut form_with_file: Multipart) -> Result<Response<Body>, AppError> {
    while let Some(field) = form_with_file.next_field().await? {
        if field.name().unwrap() == "file" {
            let cv: CV = from_str(
                std::str::from_utf8(
                    &field
                        .bytes()
                        .await
                        .with_code(StatusCode::UNPROCESSABLE_ENTITY)?,
                )
                .with_code(StatusCode::UNPROCESSABLE_ENTITY)?,
            )
            .with_code(StatusCode::UNPROCESSABLE_ENTITY)?;
            return Ok(Html(Form { init: cv }.render().unwrap()).into_response());
        }
    }
    Ok((
        StatusCode::UNPROCESSABLE_ENTITY,
        "Has to send a file in a input named \"file\"",
    )
        .into_response())
}
