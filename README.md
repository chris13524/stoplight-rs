# stoplight-rs

Stoplight code for Raspberry Pi Zero 2 W.

## Installing Raspberry Pi OS

Use [Rasberry Pi Imager](https://www.raspberrypi.com/software/) to flash Raspberry Pi OS 64-bit Lite (Legacy is fine).
- General tab:
  - Set hostname
  - Set username and password
  - Configure wireless LAN with the 2.4 Ghz WiFi network
- Set to Allow public-key authentication only and provide an SSH public key

## Building

Note if you use a 32-bit OS, you must use the `armv7-unknown-linux-musleabi` build target.

```bash
cargo install cross --git https://github.com/cross-rs/cross
cross build --release --target=aarch64-unknown-linux-musl
```

## Copy ops scripts & build output

```bash
scp target/aarch64-unknown-linux-musl/release/stoplight-rs stoplight2:
scp Dockerfile stoplight2:
scp docker-compose.yml stoplight2:
scp up.sh stoplight2:
scp default.creds stoplight2: # ngs.global credentials
```

On the Pi:

```bash
sudo apt update && sudo apt upgrade && sudo apt install -y docker.io docker-compose
sudo usermod -aG docker stoplight
# relogin
./up.sh
```
