use {
    crate::{
        light_state::LightState,
        stoplight::{self, Stoplight},
    },
    nats::kv::{Entry, Watch},
    std::convert::Infallible,
    thiserror::Error,
    tracing::{info, warn},
};

pub fn start(watch: Watch) -> Result<Infallible, Error> {
    let mut stoplight = Stoplight::new()?;

    for update in watch {
        if let Err(e) = process(&mut stoplight, update) {
            warn!("Error processing update: {e}");
        }
    }

    Err(Error::OutOfUpdates)
}

pub fn process(stoplight: &mut Stoplight, update: Entry) -> Result<(), serde_json::Error> {
    let state = serde_json::from_slice::<LightState>(&update.value)?;
    info!("New light state: {state}");

    stoplight.update(state);

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Stoplight init error {0}")]
    StoplightInitError(#[from] stoplight::Error),

    #[error("Out of updates")]
    OutOfUpdates,
}
