use std::sync::Arc;

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Contact {
    Phone(Arc<str>),
    Email(Arc<str>),
    Link(Arc<str>, Arc<str>),
}

impl Default for Contact {
    fn default() -> Self {
        let empty: Arc<str> = "".into();
        Self::Link(empty.clone(), empty)
    }
}

impl Contact {
    fn new(kind: &str, info: &str) -> Self {
        match kind {
            "email" => Self::Email(info.into()),
            "phone" => Self::Phone(info.into()),
            v => Self::Link(v.into(), info.into()),
        }
    }
}

impl From<(&str, &str)> for Contact {
    fn from(value: (&str, &str)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Template, Default, Serialize, Deserialize)]
#[serde(transparent)]
#[template(path = "components/contact.html")]
pub struct ContactList(Vec<Contact>);

impl<I> FromIterator<I> for ContactList
where
    Contact: From<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(Contact::from).collect())
    }
}
