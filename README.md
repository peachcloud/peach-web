# peach-web

[![Build Status](https://travis-ci.com/peachcloud/peach-web.svg?branch=master)](https://travis-ci.com/peachcloud/peach-web)

## Web Server for PeachCloud

**peach-web** provides a web server for serving static assets (including all client code) and a JSON API.

Initial development will focus on administration of the device itself, with SSB-related administration being integrated at a later stage.

The peach-web stack currently consists of [Rocket](https://rocket.rs/) (Rust web framework).

_Note: This is a work-in-progress._

### JSON API

All JSON API calls are prefixed by `/api/v1/`. This has been excluded from the table below to keep the table compact.

| Endpoint | Method | Parameters | Description |
| --- | --- | --- | --- |
| network/ip | GET | | Returns IP address values for wlan0 & ap0 interfaces |
| network/rssi | GET | | Returns RSSI for connected WiFi network |
| network/ssid | GET | | Returns SSID for connected WiFi network |
| network/state | GET | | Returns state of wlan0 & ap0 interfaces |
| network/status | GET | | Returns status object for connected WiFi network |
| network/wifi | GET | | Returns scan results for in-range access-points |
| network/wifi | POST | `ssid` & `pass` | Submit SSID & password to create new WiFi connection |

### Environment

The web application deployment mode is configured with the `ROCKET_ENV` environment variable:

`export ROCKET_ENV=stage`

Other deployment modes are `dev` and `prod`. Read the [Rocket Environment Configurations docs](https://rocket.rs/v0.4/guide/configuration/#environment) for further information.

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
