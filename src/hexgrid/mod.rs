use askama::Template;
mod hex;

use serde::{Deserialize, Serialize};

#[derive(Template, Default, Serialize, Deserialize)]
#[template(path = "hexgrid.html")]
#[serde(transparent)]
pub struct Hexgrid(pub Vec<hex::Hex>);

impl Hexgrid {
    pub fn from_content(content: impl IntoIterator<Item = (String, hex::Content)>) -> Self {
        Self(content.into_iter().map(hex::Hex::new).collect())
    }
}
