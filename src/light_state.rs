use {
    core::fmt,
    serde::Deserialize,
    std::fmt::{Display, Formatter},
};

#[derive(Deserialize)]
pub struct LightState {
    pub red: bool,
    pub yellow: bool,
    pub green: bool,
}

impl Display for LightState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.red {
            write!(f, "R")?;
        }
        if self.yellow {
            write!(f, "Y")?;
        }
        if self.green {
            write!(f, "G")?;
        }

        Ok(())
    }
}
