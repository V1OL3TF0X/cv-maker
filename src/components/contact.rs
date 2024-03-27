use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/contact.html")]
pub struct Contact;
