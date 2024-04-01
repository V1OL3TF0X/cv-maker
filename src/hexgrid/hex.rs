use askama::Template;
use serde::Deserialize;

#[derive(Template, Deserialize)]
#[template(path = "hex.html")]
#[serde(transparent)]
pub struct Hex<'a>(#[serde(borrow)] Content<'a>);

impl<'a> Hex<'a> {
    pub fn new(content: Content<'a>) -> Self {
        Self(content)
    }
}
#[derive(Deserialize)]
pub enum Content<'a> {
    Text(&'a str),
    Img(&'a str),
}
