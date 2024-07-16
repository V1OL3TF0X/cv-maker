use crate::{
    components::{ContactList, EducationList, ExperienceList, InterestList, Me, SkillList},
    gradient::{Gradient, NewTrait},
    hexgrid::Hexgrid,
};
use askama::Template;
use serde::{Deserialize, Serialize};

use crate::filters;

#[derive(Template, Default, Serialize, Deserialize)]
#[template(path = "cv.html")]
pub struct CV {
    #[serde(default)]
    pub gradient: Gradient,
    #[serde(default)]
    pub me: Me,
    #[serde(default)]
    pub about: String,
    #[serde(default)]
    pub education: EducationList,
    #[serde(default)]
    pub experience: ExperienceList,
    #[serde(default)]
    pub interests: InterestList,
    #[serde(default)]
    pub contact: ContactList,
    #[serde(default)]
    pub skills: SkillList,
    #[serde(default)]
    pub qualities: Hexgrid,
}
