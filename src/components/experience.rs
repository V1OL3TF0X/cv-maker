use std::fmt::Display;

use askama::Template;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub enum TimeRange {
    #[default]
    Present,
    Date(NaiveDate),
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Present => write!(f, "Present"),
            Self::Date(d) => write!(f, "{d}"),
        }
    }
}

#[derive(Default, Deserialize)]
pub struct Experience<'a> {
    from: NaiveDate,
    to: TimeRange,
    description: &'a str,
    name: &'a str,
}

impl<'a> Experience<'a> {
    pub fn new(from: NaiveDate, to: TimeRange, name: &'a str, description: &'a str) -> Self {
        Self {
            from,
            to,
            name,
            description,
        }
    }
}

impl<'a> From<(NaiveDate, TimeRange, &'a str, &'a str)> for Experience<'a> {
    fn from(value: (NaiveDate, TimeRange, &'a str, &'a str)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

#[derive(Template, Default, Deserialize)]
#[template(path = "components/experience_list.html")]
pub struct ExperienceList<'a>(#[serde(borrow)] Vec<Experience<'a>>);

impl<'a, I> FromIterator<I> for ExperienceList<'a>
where
    Experience<'a>: From<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(Experience::from).collect())
    }
}
#[derive(Template, Default, Deserialize)]
#[template(path = "components/education_list.html")]
pub struct EducationList<'a>(#[serde(borrow)] Vec<&'a str>);

impl<'a> FromIterator<&'a str> for EducationList<'a>
{
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
