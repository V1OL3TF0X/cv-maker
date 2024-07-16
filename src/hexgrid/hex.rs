use std::sync::Arc;

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Serialize, Deserialize)]
#[template(path = "hex.html")]
#[serde(transparent)]
pub struct Hex(pub Content);

impl Hex {
    pub fn data(&self) -> &str {
        match &self.0 {
            Content::Text(v) => v,
            Content::Img(v) => v,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Content {
    Text(Arc<str>),
    Img(Arc<str>),
}
