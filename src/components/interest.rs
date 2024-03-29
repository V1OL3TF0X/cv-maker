use anyhow::Result;
use askama::Template;

#[derive(Template, Default)]
#[template(path = "components/interest_list.html"t)]
pub struct InterestList<'a>(Vec<Interest<'a>>);

pub struct MaybeInterestList<'a>(pub Result<InterestList<'a>>);

impl<'a> FromIterator<&'a str> for MaybeInterestList<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(Interest::try_from)
                .collect::<Result<Vec<_>>>()
                .map(InterestList),
        )
    }
}

impl<'a> TryFrom<&'a str> for Interest<'a> {
    type Error = anyhow::Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // TODO - add URL validation
        Ok(Self {
            interest_icon: value,
        })
    }
}

struct Interest<'a> {
    interest_icon: &'a str,
}
