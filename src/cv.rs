use askama::Template;
use askama_axum::IntoResponse;
use axum::response::Html;

use crate::{components::{Contact, ExperienceList, Interest, Me, Skill},hexgrid::{
    hex::Content,
    Hexgrid,
}};

pub const BG_ID: &str = "bg-gradient";

#[derive(Template, Default)]
#[template(path = "cv.html")]
struct CV<'a> {
    me: Me<'a>,
    about: &'a str,
    experience: ExperienceList,
    interest: Vec<Interest>,
    contact: Contact,
    skills: Vec<Skill>,
    capabilities: Hexgrid,
}

pub async fn cv_page() -> impl IntoResponse {
    Html(
        CV {
            me: Me::new("https://picsum.photos/200/300", "My title", "My Description"),
            capabilities: Hexgrid::from_content(vec![
                Content::Text("Something".into()),
                Content::Text("Something else".into()),
                Content::Text("Something different".into()),
                Content::Text("Something weird".into()),
                Content::Img("https://picsum.photos/200/300".into()),
            ]),
            ..Default::default()
        }
        .render()
        .unwrap(),
    )
}
