use std::{fmt::Display, sync::Arc};

use anyhow::Result;
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Default, Serialize, Deserialize)]
#[template(path = "components/interest_list.html"t)]
#[serde(transparent)]
pub struct InterestList(pub Vec<Interest>);

impl TryFrom<&str> for Interest {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO - add URL validation
        Ok(Self {
            interest_icon: value.into(),
        })
    }
}
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Interest {
    pub interest_icon: Arc<str>,
}

impl Display for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.interest_icon.fmt(f)
    }
}
