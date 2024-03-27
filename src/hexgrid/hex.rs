use askama::Template;

#[derive(Template)]
#[template(path = "hex.html")]
pub struct Hex(Content);

impl Hex {
    pub fn new(content: Content) -> Self {
        Self(content)
    }
}

pub enum Content {
    Text(String),
    Img(String),
}
