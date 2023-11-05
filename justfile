fmt:
  cargo +nightly fmt -- --config group_imports=StdExternalCrate,imports_granularity=One

clippy:
  cross clippy --all-features --tests -- -D warnings

lint: fmt clippy

build:
  cross build --release
