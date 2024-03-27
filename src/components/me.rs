use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/me.html")]
pub struct Me;
