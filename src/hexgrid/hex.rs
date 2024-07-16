use std::sync::Arc;

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Serialize, Deserialize)]
#[template(path = "hex.html")]
pub struct Hex {
    kind: Content,
    pub value: Arc<str>,
}

impl Hex {
    pub fn new((value, kind): (String, Content)) -> Self {
        Self {
            value: value.into(),
            kind,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Content {
    Text,
    Img,
}
