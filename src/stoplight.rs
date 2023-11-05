use {
    crate::light_state::LightState,
    rppal::{
        gpio,
        gpio::{Gpio, Level, OutputPin},
    },
    thiserror::Error,
};

// https://www.raspberrypi.com/documentation/computers/os.html#gpio-and-the-40-pin-header
const RED_PIN: u8 = 6; // physical pin 31
const YELLOW_PIN: u8 = 13; // physical pin 33
const GREEN_PIN: u8 = 19; // physical pin 37

pub struct Stoplight {
    red: OutputPin,
    yellow: OutputPin,
    green: OutputPin,
}

impl Stoplight {
    pub fn new() -> Result<Self, Error> {
        let gpio = Gpio::new()?;

        let red = gpio.get(RED_PIN)?.into_output();
        let yellow = gpio.get(YELLOW_PIN)?.into_output();
        let green = gpio.get(GREEN_PIN)?.into_output();

        Ok(Self { red, yellow, green })
    }

    pub fn update(&mut self, state: LightState) {
        fn state_to_level(state: bool) -> Level {
            if state {
                Level::High
            } else {
                Level::Low
            }
        }

        self.red.write(state_to_level(state.red));
        self.yellow.write(state_to_level(state.yellow));
        self.green.write(state_to_level(state.green));
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("GPIO error {0}")]
    GpioError(#[from] gpio::Error),
}
