use {
    nats::{jetstream, kv::Config},
    std::io::ErrorKind,
    thiserror::Error,
    tracing_subscriber::FmtSubscriber,
};

mod control;
mod light_state;
mod stoplight;

fn main() -> Result<(), Error> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())
        .expect("Failed to set_global_default");

    let nats = nats::Options::with_credentials("default.creds")
        .max_reconnects(None) // default is 60
        .tls_required(true)
        .connect("connect.ngs.global")?;
    let js = jetstream::new(nats);

    let bucket = "stoplight";
    let kv = match js.key_value(bucket) {
        Ok(kv) => kv,
        Err(e) => {
            if e.kind() == ErrorKind::InvalidInput {
                js.create_key_value(&Config {
                    bucket: bucket.to_owned(),
                    history: 5,
                    max_bytes: 1000,
                    ..Default::default()
                })?;
            }
            Err(e)?
        }
    };

    let watch = kv.watch_all()?;
    control::start(watch)?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("NATS error: {0}")]
    NatsError(#[from] std::io::Error),

    #[error("Control error: {0}")]
    ControlError(#[from] control::Error),
}
