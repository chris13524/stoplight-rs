use {
    async_nats::jetstream::{self, context::KeyValueErrorKind, kv::Config},
    thiserror::Error,
    tracing_subscriber::FmtSubscriber,
};

mod control;
mod light_state;
mod stoplight;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())
        .expect("Failed to set_global_default");

    let nats = async_nats::ConnectOptions::with_credentials_file("default.creds")
        .await
        .map_err(Error::NatsCredentialsFile)?
        .require_tls(true)
        .connect("connect.ngs.global")
        .await
        .map_err(Error::NatsConnect)?;
    let js = jetstream::new(nats);

    let bucket = "stoplight";
    let kv = match js.get_key_value(bucket).await {
        Ok(kv) => kv,
        Err(e) => {
            if e.kind() == KeyValueErrorKind::GetBucket {
                js.create_key_value(Config {
                    bucket: bucket.to_owned(),
                    history: 5,
                    max_bytes: 1000,
                    ..Default::default()
                })
                .await
                .map_err(Error::NatsCreateKv)?;
            }
            Err(Error::NatsGetKv(e))?
        }
    };

    let watch = kv.watch_all().await.map_err(Error::NatsKvWatch)?;
    control::start(watch).await.map_err(Error::ControlError)?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("NATS credentials file error: {0}")]
    NatsCredentialsFile(tokio::io::Error),

    #[error("NATS connect error: {0}")]
    NatsConnect(async_nats::ConnectError),

    #[error("NATS get KV error: {0}")]
    NatsGetKv(async_nats::jetstream::context::KeyValueError),

    #[error("NATS create KV error: {0}")]
    NatsCreateKv(async_nats::jetstream::context::CreateKeyValueError),

    #[error("NATS watch KV error: {0}")]
    NatsKvWatch(async_nats::jetstream::kv::WatchError),

    #[error("Control error: {0}")]
    ControlError(control::Error),
}
