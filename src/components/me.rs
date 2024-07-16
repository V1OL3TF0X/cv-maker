use std::sync::Arc;

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Serialize, Deserialize)]
#[template(path = "components/me.html")]
pub struct Me {
    pub photo_url: Arc<str>,
    pub title: Arc<str>,
    pub name: Arc<str>,
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

impl Me {
    pub fn new(url: &str, name: &str, title: &str) -> Self {
        Self {
            photo_url: Arc::from(url),
            title: Arc::from(title),
            name: Arc::from(name),
        }
    }
}
