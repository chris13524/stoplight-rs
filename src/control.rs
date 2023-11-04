use crate::{gpio::update_gpio, lights::Lights};
use nats::kv::{Entry, Watch};
use thiserror::Error;
use tracing::{info, warn};

pub fn start(watch: Watch) {
    for update in watch {
        if let Err(e) = process(update) {
            warn!("Error processing update: {e}");
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to deserialize lights update {0}")]
    LightsDeserialization(#[from] serde_json::Error),

    #[error("Error updating GPIO {0}")]
    GpioError(#[from] crate::gpio::Error),
}

pub fn process(update: Entry) -> Result<(), Error> {
    let lights = serde_json::from_slice::<Lights>(&update.value)?;
    info!("New lights: {lights}");

    update_gpio(lights)?;

    Ok(())
}
