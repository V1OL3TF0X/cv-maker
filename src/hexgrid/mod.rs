use askama::Template;

use self::hex::Content;

pub mod hex;

#[derive(Template, Default)]
#[template(path = "hexgrid.html")]
pub struct Hexgrid(Vec<hex::Hex>);

impl Hexgrid {
    pub fn from_content(content: impl IntoIterator<Item = Content>) -> Self {
        Self(content.into_iter().map(hex::Hex::new).collect())
    }
}
