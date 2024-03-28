use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/me.html")]
pub struct Me<'a> {
    photo_url: &'a str,
    title: &'a str,
    name: &'a str
}

impl<'a> Me<'a> {
    pub fn new(url: &'a str, name: &'a str, title: &'a str) -> Self {
        Self {
            photo_url: url,
            title,
            name,
        }
    }
}
