use anyhow::Result;
use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/skill_list.html")]
pub struct SkillList<'a> {
    skills: Vec<Skill<'a>>,
    summary: &'a str,
}

#[derive(Default)]
pub struct Skill<'a> {
    name: &'a str,
    experience: usize,
}

impl<'a> SkillList<'a> {
    pub fn from_list(
        list: impl IntoIterator<Item = (&'a str, usize)>,
        summary: &'a str,
    ) -> Result<Self> {
        Ok(Self {
            skills: list
                .into_iter()
                .map(Skill::try_from)
                .collect::<Result<Vec<_>>>()?,
            summary,
        })
    }
}

impl<'a> TryFrom<(&'a str, usize)> for Skill<'a> {
    type Error = anyhow::Error;

    fn try_from((name, experience): (&'a str, usize)) -> Result<Self, Self::Error> {
        Self::new(name, experience)
    }
}

impl<'a> Skill<'a> {
    pub fn new(name: &'a str, experience: usize) -> Result<Self> {
        if experience > 100 {
            return Err(anyhow::anyhow!(
                "experience has to be a value between 0 and 100"
            ));
        }
        Ok(Self { name, experience })
    }
}
