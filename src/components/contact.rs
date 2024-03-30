use askama::Template;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct Contact<'a> {
    kind: &'a str,
    info: &'a str,
}

impl<'a> Contact<'a> {
    fn new(kind: &'a str, info: &'a str) -> Self {
        Self { kind, info }
    }
}

impl<'a> From<(&'a str, &'a str)> for Contact<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Template, Default, Deserialize)]
#[template(path = "components/contact.html")]
pub struct ContactList<'a>(#[serde(borrow)] Vec<Contact<'a>>);

impl<'a, I> FromIterator<I> for ContactList<'a>
where
    Contact<'a>: From<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(Contact::from).collect())
    }
}
