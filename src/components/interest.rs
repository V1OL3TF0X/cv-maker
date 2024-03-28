use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/interest.html"t)]
pub struct Interest;
