use std::sync::Arc;

use crate::filters;
use anyhow::Result;
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Serialize, Deserialize)]
#[template(path = "components/skill_list.html")]
pub struct SkillList {
    skills: Vec<Skill>,
    summary: Arc<str>,
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    name: Arc<str>,
    experience: usize,
}

impl Default for SkillList {
    fn default() -> Self {
        Self {
            skills: Default::default(),
            summary: "".into(),
        }
    }
}

impl SkillList {
    pub fn from_list<'a>(
        list: impl IntoIterator<Item = (&'a str, usize)>,
        summary: &str,
    ) -> Result<Self> {
        Ok(Self {
            skills: list
                .into_iter()
                .map(Skill::try_from)
                .collect::<Result<Vec<_>>>()?,
            summary: summary.into(),
        })
    }
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            name: "".into(),
            experience: 0,
        }
    }
}

impl TryFrom<(&str, usize)> for Skill {
    type Error = anyhow::Error;

    fn try_from((name, experience): (&str, usize)) -> Result<Self, Self::Error> {
        Self::new(name, experience)
    }
}

impl Skill {
    pub fn new(name: &str, experience: usize) -> Result<Self> {
        if experience > 100 {
            return Err(anyhow::anyhow!(
                "experience has to be a value between 0 and 100"
            ));
        }
        Ok(Self {
            name: name.into(),
            experience,
        })
    }
}
