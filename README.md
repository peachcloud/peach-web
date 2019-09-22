# peach-web

[![Build Status](https://travis-ci.com/peachcloud/peach-web.svg?branch=master)](https://travis-ci.com/peachcloud/peach-web)

## Web Admin Interface for PeachCloud

**peach-web** provides a web interface for monitoring and interacting with the PeachCloud device. This will allow administration of the single-board computer (ie. Raspberry Pi) running PeachCloud, as well as the ssb-server and related plugins.

Initial development will focus on administration of the device itself, with SSB-related administration being integrated at a later stage.

The peach-web stack currently consists of [Rocket](https://rocket.rs/) (Rust web framework), [Tera](https://tera.netlify.com/docs/installation/) (Rust template engine inspired by Jinja2 and the Django template language) and [Tachyons](https://tachyons.io/) (functional CSS library for humans).

_Note: This is a work-in-progress._

### JSON API

| Endpoint | Method | Parameters | Description |
| --- | --- | --- | --- |
| /api/ip | GET | | Returns IP address values for wlan0 & ap0 interfaces |
| /api/ssid | GET | | Returns SSID for connected WiFi network |
| /api/add_wifi | POST | `ssid` & `pass` | Submit SSID & password to create new WiFi connection |

### Environment

The web application deployment mode is configured with the `ROCKET_ENV` environment variable:

`export ROCKET_ENV=stage`

Other deployment modes are `dev` and `prod`. Read the [Rocket Environment Configurations docs](https://rocket.rs/v0.4/guide/configuration/#environment) for further information.

The [Tera](https://tera.netlify.com/) template directory must be configured with the `ROCKET_TEMPLATE_DIR` environment variable:

`export ROCKET_TEMPLATE_DIR=static/templates/`

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
