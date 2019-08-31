# peach-web

[![Build Status](https://travis-ci.com/peachcloud/peach-web.svg?branch=master)](https://travis-ci.com/peachcloud/peach-web)

## Web Admin Interface for PeachCloud

**peach-web** provides a web interface for monitoring and interacting with the PeachCloud device. This will allow administration of the single-board computer (ie. Raspberry Pi) running PeachCloud, as well as the ssb-server and related plugins.

Initial development will focus on administration of the device itself, with SSB-related administration being integrated at a later stage.

_Note: This is a work-in-progress._

### JSON API

| Endpoint | Method | Parameters | Description |
| --- | --- | --- | --- |
| /ip | GET | | Returns IP address values for wlan0 & ap0 interfaces |
| /ssid | GET | | Returns SSID for connected WiFi network |
| /add_wifi | POST | `ssid` & `pass` | Submit SSID & password to create new WiFi connection |

### Environment

The WebSocket server port can be configured with `PEACH_WEB_WS` environment variable:

`export PEACH_WEB_WS=2333`

When not set, the value defaults to `5115`.

Logging is made available with `env_logger`:

`export RUST_LOG=info`

Other logging levels include `debug`, `warn` and `error`.

### Setup

Clone this repo:

`git clone https://github.com/peachcloud/peach-web.git`

Move into the repo and compile:

`cd peach-web`  
`cargo build --release`

Run the tests:

`cargo test`

Run the binary:

`./target/release/peach-web`

_Note: Networking functionality requires peach-network microservice to be running._

### Licensing

AGPL-3.0
