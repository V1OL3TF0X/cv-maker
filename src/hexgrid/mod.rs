use askama::Template;
mod hex;

use serde::{Deserialize, Serialize};

#[derive(Template, Default, Serialize, Deserialize)]
#[template(path = "hexgrid.html")]
#[serde(transparent)]
pub struct Hexgrid(pub Vec<hex::Hex>);
