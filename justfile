fmt:
  cargo +nightly fmt -- --config group_imports=StdExternalCrate,imports_granularity=One

clippy:
  cargo clippy --tests -- -D warnings

test:
  cargo test

lint: fmt clippy test

build:
  cross build --release --features=rpi

devloop: lint build
