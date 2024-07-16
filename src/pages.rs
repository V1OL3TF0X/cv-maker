use crate::{
    components::{Contact, TimeRange},
    cv::CV,
    AppError,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::Multipart,
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::{Html, Response},
};
use serde_qs::axum::QsQuery;
use serde_yaml::from_str;

pub async fn cv(QsQuery(cv): QsQuery<CV>) -> impl IntoResponse {
    Html(cv.render().unwrap())
}
pub async fn cv_data(QsQuery(cv): QsQuery<CV>) -> Response<Body> {
    let contents = serde_yaml::to_string(&cv).expect("couldn't serialize CV data to TOML.");
    let body = Body::from(contents);
    Response::builder()
        .header(CONTENT_TYPE, "text/yaml; charset=utf-8")
        .header(CONTENT_DISPOSITION, "attachment; filename=\"CV.yaml\"")
        .body(body)
        .unwrap()
}

#[derive(Template)]
#[template(path = "form.html")]
struct Form {
    init: CV,
}

#[derive(Template)]
#[template(path = "form_page.html")]
struct FormPage;

pub async fn form() -> impl IntoResponse {
    Html(FormPage.render().unwrap())
}

pub async fn form_from_file(mut form_with_file: Multipart) -> Result<Response<Body>, AppError> {
    while let Some(field) = form_with_file.next_field().await? {
        if field.name().unwrap() == "file" {
            let cv: CV = from_str(std::str::from_utf8(&field.bytes().await?)?)?;
            return Ok(Html(Form { init: cv }.render().unwrap()).into_response());
        }
    }
    Ok(Html(String::from("")).into_response())
}
