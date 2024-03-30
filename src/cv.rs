use askama::Template;
use askama_axum::IntoResponse;
use axum::response::Html;
use chrono::NaiveDate;

use crate::{
    components::{
        Contact, ExperienceList, InterestList, MaybeInterestList, Me, SkillList, TimeRange,
    },
    hexgrid::{Content, Hexgrid},
};

pub const BG_ID: &str = "bg-gradient";

#[derive(Template, Default)]
#[template(path = "cv.html")]
struct CV<'a> {
    me: Me<'a>,
    about: &'a str,
    experience: ExperienceList<'a>,
    interests: InterestList<'a>,
    contact: Contact,
    skills: SkillList<'a>,
    capabilities: Hexgrid,
}

pub async fn cv_page() -> impl IntoResponse {
    Html(
        CV {
            me: Me::new("https://picsum.photos/200/300", "My name", "My title"),
            about: "Lorem ipsum dolor sit amet. asdhjkashdkjasdhjkashdk sdas asdas asdsa asd asd asdasdas as asdas adas sas as.<br><br>askldjj l klasjdlk  kjalsl lkasj slkasjd aslkdj alksdj alksjd lkassj  aklsjdl ksjalksj  aksjl jklajld s",
            skills: SkillList::from_list(vec![
                ("Html/CSS", 60),
                ("React", 85),
                ("Vue", 30),
                ("Rust", 50),
                ("Typescript", 40),
                ("PHP", 30),
                ("Git", 90),
            ], "Lorem ipsum dolor sit amet. asdhjkashdkjasdhjkashdk sdas asdas asdsa asd asd asdasdas as asdas adas sas as.<br><br>askldjj l klasjdlk  kjalsl lkasj slkasjd aslkdj alksdj alksjd lkassj  aklsjdl ksjalksj  aksjl jklajld s",
).unwrap(),
            experience: ExperienceList::from_iter(vec![
                (NaiveDate::from_ymd(2021, 11, 30), TimeRange::Present, "Frontend Developer", "Lorem ipsum dolor sit amet. kfdas sakldj as ask daj a kl aa lksasd "),
                (NaiveDate::from_ymd(2021, 11, 30), TimeRange::Present, "Frontend Developer", "Lorem ipsum dolor sit amet. kfdas sakldj as ask daj a kl aa lksasd "),
                (NaiveDate::from_ymd(2021, 11, 30), TimeRange::Present, "Frontend Developer", "Lorem ipsum dolor sit amet. kfdas sakldj as ask daj a kl aa lksasd "),
                (NaiveDate::from_ymd(2021, 11, 30), TimeRange::Present, "Frontend Developer", "Lorem ipsum dolor sit amet. kfdas sakldj as ask daj a kl aa lksasd "),
                (NaiveDate::from_ymd(2021, 11, 30), TimeRange::Present, "Frontend Developer", "Lorem ipsum dolor sit amet. kfdas sakldj as ask daj a kl aa lksasd "),
                (NaiveDate::from_ymd(2021, 11, 30), TimeRange::Present, "Frontend Developer", "Lorem ipsum dolor sit amet. kfdas sakldj as ask daj a kl aa lksasd "),
            ]),
            interests: vec![
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
            ].into_iter().collect::<MaybeInterestList>().0.unwrap(),
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
