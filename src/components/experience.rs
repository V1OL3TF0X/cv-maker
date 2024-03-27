use askama::Template;

#[derive(Default)]
struct Experience;

#[derive(Template, Default)]
#[template(path = "components/experience_list.html")]
pub struct ExperienceList(Vec<Experience>);
