use askama::Template;
use askama_axum::IntoResponse;
use axum::response::Html;
use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    components::{
        ContactList, ExperienceList, EducationList, InterestList, MaybeInterestList, Me, SkillList, TimeRange,
    },
    hexgrid::{Content, Hexgrid},
};

pub const BG_ID: &str = "bg-gradient";
#[derive(Deserialize)]
struct Gradient<'a> {
    start: &'a str,
    end: &'a str,
}

impl<'a> Default for Gradient<'a> {
    fn default() -> Self {
        Self {
            start: "#B008C4",
            end: "#FE7500",
        }
    }
}

#[derive(Template, Default, Deserialize)]
#[template(path = "cv.html")]
struct CV<'g, 'cn, 'ci, 'e, 's, 'm, 'a, 'i, 'h> {
    #[serde(borrow)]
    gradient: Gradient<'g>,
    #[serde(borrow)]
    me: Me<'m>,
    about: &'a str,
    #[serde(borrow)]
    education: EducationList<'e>,
    #[serde(borrow)]
    experience: ExperienceList<'e>,
    #[serde(borrow)]
    interests: InterestList<'i>,
    #[serde(borrow)]
    contact: ContactList<'ci, 'cn>,
    #[serde(borrow)]
    skills: SkillList<'s>,
    #[serde(borrow)]
    qualities: Hexgrid<'h>,
}

fn htmlize(s: &str) -> String {
    s.replace('\n', "<br>")
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
            interests: MaybeInterestList::from_iter(vec![
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
                "/assets/github-cat.svg",
            ]).0.unwrap(),
            contact: ContactList::from_iter(vec![
                ("Phone", "+48 123 456 789"),
                ("email", "email@example.com"),
                ("github", "github.com/username"),
            ]),
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
