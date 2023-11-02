# stoplight-rs

Stoplight code for Raspberry Pi Zero 2 W.

```bash
cross build --release --target=armv7-unknown-linux-musleabi
scp target/armv7-unknown-linux-musleabi/release/stoplight-rs stoplight2:stoplight/stoplight-rs
```
