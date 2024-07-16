use crate::{components::TimeRange, cv::CV};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    body::Body,
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::{Html, Response},
};
use serde_qs::axum::QsQuery;

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
#[template(path = "form.html", print = "code")]
struct Form {
    init: CV,
}

pub async fn form() -> impl IntoResponse {
    Html(
        Form {
            init: CV::default(),
        }
        .render()
        .unwrap(),
    )
}
