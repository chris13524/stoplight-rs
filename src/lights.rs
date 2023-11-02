use core::fmt;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct Lights {
    pub red: bool,
    pub yellow: bool,
    pub green: bool,
}

impl Display for Lights {
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
