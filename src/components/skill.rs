use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/skill.html")]
pub struct Skill;
