use {
    crate::{
        light_state::LightState,
        stoplight::{self, Stoplight},
    },
    async_nats::jetstream::kv::{Entry, Watch},
    futures::StreamExt,
    std::convert::Infallible,
    thiserror::Error,
    tracing::{info, warn},
};

pub async fn start(mut watch: Watch<'_>) -> Result<Infallible, Error> {
    let mut stoplight = Stoplight::new().map_err(Error::StoplightInit)?;

    while let Some(update) = watch.next().await {
        let update = update.map_err(Error::NatsWatcher)?;
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
    StoplightInit(stoplight::Error),

    #[error("NATS watcher error {0}")]
    NatsWatcher(async_nats::jetstream::kv::WatcherError),

    #[error("Out of updates")]
    OutOfUpdates,
}
