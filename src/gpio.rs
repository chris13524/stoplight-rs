use crate::lights::Lights;
use rppal::gpio;
use rppal::gpio::Gpio;
use rppal::gpio::Level::*;
use thiserror::Error;

// https://www.raspberrypi.com/documentation/computers/os.html#gpio-and-the-40-pin-header
const RED_PIN: u8 = 6; // physical pin 31
const YELLOW_PIN: u8 = 13; // physical pin 33
const GREEN_PIN: u8 = 19; // physical pin 37

pub fn update_gpio(lights: Lights) -> Result<(), Error> {
    let gpio = Gpio::new()?;
    let mut red_pin = gpio.get(RED_PIN)?.into_output();
    red_pin.set_reset_on_drop(false);
    let mut yellow_pin = gpio.get(YELLOW_PIN)?.into_output();
    yellow_pin.set_reset_on_drop(false);
    let mut green_pin = gpio.get(GREEN_PIN)?.into_output();
    green_pin.set_reset_on_drop(false);

    red_pin.write(if lights.red { High } else { Low });
    yellow_pin.write(if lights.yellow { High } else { Low });
    green_pin.write(if lights.green { High } else { Low });

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("GPIO error {0}")]
    GpioError(#[from] gpio::Error),
}
