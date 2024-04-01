use askama::Template;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Contact<'a, 'b> {
    Phone(&'a str),
    Email(&'a str),
    Link(&'b str, &'a str),
}

impl<'a, 'b> Default for Contact<'a, 'b> {
    fn default() -> Self {
        Self::Link("", "")
    }
}

impl<'a, 'b> Contact<'a, 'b> {
    fn new(kind: &'b str, info: &'a str) -> Self {
        match kind {
            "email" => Self::Email(info),
            "phone" => Self::Phone(info),
            v => Self::Link(v, info),
        }
    }
}

impl<'a, 'b> From<(&'b str, &'a str)> for Contact<'a, 'b> {
    fn from(value: (&'b str, &'a str)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Template, Default, Deserialize)]
#[serde(transparent)]
#[template(path = "components/contact.html")]
pub struct ContactList<'a, 'b>(#[serde(borrow)] Vec<Contact<'a, 'b>>);

impl<'a, 'b, I> FromIterator<I> for ContactList<'a, 'b>
where
    Contact<'a, 'b>: From<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(Contact::from).collect())
    }
}
