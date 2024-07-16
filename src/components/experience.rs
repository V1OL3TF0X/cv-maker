use std::fmt::Display;
use std::sync::Arc;

use crate::filters;
use askama::Template;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Experience {
    pub from: NaiveDate,
    pub to: TimeRange,
    pub description: Arc<str>,
    pub name: Arc<str>,
}

impl Default for Experience {
    fn default() -> Self {
        let empty: Arc<str> = "".into();
        Self {
            from: Default::default(),
            to: Default::default(),
            description: empty.clone(),
            name: empty,
        }
    }
}

impl Experience {
    pub fn new(from: NaiveDate, to: TimeRange, name: &str, description: &str) -> Self {
        Self {
            from,
            to,
            name: Arc::from(name),
            description: Arc::from(description),
        }
    }
}

impl From<(NaiveDate, TimeRange, &str, &str)> for Experience {
    fn from(value: (NaiveDate, TimeRange, &str, &str)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

#[derive(Template, Default, Serialize, Deserialize)]
#[serde(transparent)]
#[template(path = "components/experience_list.html")]
pub struct ExperienceList(pub Vec<Experience>);

impl<I> FromIterator<I> for ExperienceList
where
    Experience: From<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(Experience::from).collect())
    }
}
#[derive(Template, Default, Serialize, Deserialize)]
#[serde(transparent)]
#[template(path = "components/education_list.html")]
pub struct EducationList(pub Vec<Arc<str>>);

impl<'a> FromIterator<&'a str> for EducationList {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self(iter.into_iter().map(Arc::from).collect())
    }
}
