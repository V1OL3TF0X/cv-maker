use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/interest.html")]
pub struct Interest;
