use askama::Template;
mod hex;

pub use hex::*;
use serde::Deserialize;

#[derive(Template, Default, Deserialize)]
#[template(path = "hexgrid.html")]
#[serde(transparent)]
pub struct Hexgrid<'a>(#[serde(borrow)] Vec<hex::Hex<'a>>);

impl<'a> Hexgrid<'a> {
    pub fn from_content(content: impl IntoIterator<Item = hex::Content<'a>>) -> Self {
        Self(content.into_iter().map(hex::Hex::new).collect())
    }
}
