use serde::{Deserialize, Serialize};

pub const BG_ID: &str = "bg-gradient";
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Gradient(pub Vec<GradientStop>);

#[derive(Serialize, Deserialize)]
pub struct GradientStop {
    pub color: Box<str>,
    pub stop: Option<f64>,
}

impl GradientStop {
    pub fn stop_v(&self) -> String {
        if let Some(v) = self.stop {
            v.to_string()
        } else {
            "".into()
        }
    }
}

impl Default for GradientStop {
    fn default() -> Self {
        Self {
            color: "".into(),
            stop: None,
        }
    }
}

impl From<&str> for GradientStop {
    fn from(color: &str) -> Self {
        Self {
            color: Box::from(color),
            stop: None,
        }
    }
}

impl From<(&str, Option<f64>)> for GradientStop {
    fn from((color, stop): (&str, Option<f64>)) -> Self {
        Self {
            color: Box::from(color),
            stop,
        }
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self::from_iter(vec![("#B008C4", None), ("#FE7500", None)])
    }
}

pub trait NewTrait {
    fn to_var(&self, i: usize) -> String;
    fn stop_attrs(&self, i: &usize, total: usize) -> String;
}

impl NewTrait for GradientStop {
    fn to_var(&self, i: usize) -> String {
        format!("--gradient-{i}: {color}", color = self.color)
    }
    fn stop_attrs(&self, i: &usize, total: usize) -> String {
        let i = *i - 1;
        let offset = self.stop.unwrap_or(i as f64 / (total - 1) as f64);
        format!("offset=\"{offset}\" stop-color=\"var(--gradient-{i})\"")
    }
}

impl<I> FromIterator<I> for Gradient
where
    GradientStop: From<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(GradientStop::from).collect())
    }
}

impl Gradient {
    pub fn css_vars(&self) -> String {
        let (all_vars, var_defs): (Vec<_>, Vec<_>) = self
            .0
            .iter()
            .enumerate()
            .map(|(i, stop)| {
                (
                    format!(
                        "var(--gradient-{i}) {stop_pr}",
                        stop_pr = stop
                            .stop
                            .map(|s| format!("{}% ", s * 100.0))
                            .unwrap_or_default()
                    ),
                    stop.to_var(i),
                )
            })
            .unzip();
        format!(
            "{}; --gradient-stops: {};",
            var_defs.join("; "),
            all_vars.join(", ")
        )
    }
}
