use askama::Template;
use askama_axum::IntoResponse;
use axum::response::Html;
use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    components::{
        ContactList, EducationList, ExperienceList, InterestList, MaybeInterestList, Me, SkillList,
        TimeRange,
    },
    hexgrid::{Content, Hexgrid},
};

pub const BG_ID: &str = "bg-gradient";
#[derive(Deserialize)]
#[serde(transparent)]
struct Gradient<'a>(#[serde(borrow)] Vec<GradientStop<'a>>);

#[derive(Deserialize)]
struct GradientStop<'a> {
    color: &'a str,
    stop: Option<f64>,
}

impl<'a> From<&'a str> for GradientStop<'a> {
    fn from(color: &'a str) -> Self {
        Self { color, stop: None }
    }
}

impl<'a> From<(&'a str, Option<f64>)> for GradientStop<'a> {
    fn from((color, stop): (&'a str, Option<f64>)) -> Self {
        Self { color, stop }
    }
}

impl<'a> Default for Gradient<'a> {
    fn default() -> Self {
        Self::from_iter(vec![("#B008C4", None), ("#FE7500", None)])
    }
}

impl<'a> GradientStop<'a> {
    fn to_var(&self, i: usize) -> String {
        format!("--gradient-{i}: {color}", color = self.color)
    }
    fn stop_attrs(&self, i: &usize, total: usize) -> String {
        let i = *i - 1;
        let offset = self.stop.unwrap_or(i as f64 / (total - 1) as f64);
        format!("offset=\"{offset}\" stop-color=\"var(--gradient-{i})\"")
    }
}

impl<'a, I> FromIterator<I> for Gradient<'a>
where
    GradientStop<'a>: From<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(GradientStop::from).collect())
    }
}

impl<'a> Gradient<'a> {
    fn css_vars(&self) -> String {
        let (all_vars, var_defs): (Vec<_>, Vec<_>) = self
            .0
            .iter()
            .enumerate()
            .map(|(i, stop)| {
                (
                    format!(
                        "var(--gradient-{i}) {stop_pr}",
                        stop_pr = stop
                            .stop
                            .map(|s| format!("{}% ", s * 100.0))
                            .unwrap_or_default()
                    ),
                    stop.to_var(i),
                )
            })
            .unzip();
        format!(
            "{}; --gradient-stops: {};",
            var_defs.join("; "),
            all_vars.join(", ")
        )
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
    let re = regex::Regex::new(r"\b([^ ]) ").unwrap();
    re.replace_all(s, "$1&nbsp;").replace('\n', "<br>")
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
