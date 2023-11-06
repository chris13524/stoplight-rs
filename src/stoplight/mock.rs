use {crate::light_state::LightState, thiserror::Error};

pub struct Stoplight {}

impl Stoplight {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn update(&mut self, _state: LightState) {}
}

#[derive(Error, Debug)]
pub enum Error {}
