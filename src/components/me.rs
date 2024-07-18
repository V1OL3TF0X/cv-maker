use std::sync::Arc;

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Serialize, Deserialize)]
#[template(path = "components/me.html")]
pub struct Me {
    #[serde(default = "blank_photo")]
    pub photo_url: Arc<str>,
    pub title: Arc<str>,
    pub name: Arc<str>,
}

fn blank_photo() -> Arc<str> {
    "/assets/nice.jpg".into()
}

impl Default for Me {
    fn default() -> Self {
        let empty: Arc<str> = "".into();
        Self {
            photo_url: empty.clone(),
            title: empty.clone(),
            name: empty,
        }
    }
}
