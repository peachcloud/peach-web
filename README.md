# peach-web

[![Build Status](https://travis-ci.com/peachcloud/peach-web.svg?branch=master)](https://travis-ci.com/peachcloud/peach-web)

## Web Interface for PeachCloud

**peach-web** provides a web interface for the PeachCloud device. It serves static assets and exposes a JSON API for programmatic interactions.

Initial development is focused on administration of the device itself, beginning with networking functionality, with SSB-related administration to be integrated at a later stage.

The peach-web stack currently consists of [Rocket](https://rocket.rs/) (Rust web framework), [Tera](http://tera.netlify.com/) (Rust template engine), HTML, CSS and JavaScript.


_Note: This is a work-in-progress._

### WEB APP ROUTES

| Endpoint | Method | Parameters | Description |
| --- | --- | --- | --- |
| `/` | GET | | Home |
| `/network` | GET | | Network status overview |
| `/network/ap/activate` | GET | | Activate WiFi access point mode |
| `/network/wifi` | GET | | List of networks |
| `/network/wifi?<ssid>` | GET | `ssid` | Details of a single network |
| `/network/wifi/activate` | GET | | Activate WiFi client mode |
| `/network/wifi/add` | GET | `ssid` (optional - prepopulation value of SSID in form) | Add a WiFi network |
| `/network/wifi/add` | POST | `ssid` & `pass` | Submit form to add a WiFi network |
| `/network/wifi/forget` | POST | `ssid` | Submit form to forget a saved WiFi network |

### JSON API

All JSON API calls are prefixed by `/api/v1/`. This has been excluded from the table below to keep the table compact.

| Endpoint | Method | Parameters | Description |
| --- | --- | --- | --- |
| `network/activate_ap` | POST | | Activate WiFi access point mode |
| `network/activate_client` | POST | | Activate WiFi client mode |
| `network/ip` | GET | | Returns IP address values for wlan0 & ap0 interfaces |
| `network/rssi` | GET | | Returns RSSI for connected WiFi network |
| `network/ssid` | GET | | Returns SSID for connected WiFi network |
| `network/state` | GET | | Returns state of wlan0 & ap0 interfaces |
| `network/status` | GET | | Returns status object for connected WiFi network |
| `network/wifi` | GET | | Returns scan results for in-range access-points |
| `network/wifi` | POST | `ssid` & `pass` | Submit SSID & password to create new WiFi connection |
| `network/wifi/forget` | POST | `ssid` | Submit SSID to delete credentials for given WiFi network |
| `ping` | GET | | Returns `pong!` |

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

### Design

`peach-web` is built on the Rocket webserver and Tera templating engine. It presents a web interface for interacting with the device. HTML is rendered server-side. Request handlers call JSON-RPC microservices and serve HTML and assets. A JSON API is exposed for remote calls and dynamic client-side content updates (via vanilla JavaScript following unobstructive design principles). Each Tera template is passed a context object. In the case of Rust, this object is a `struct` and must implement `Serialize`. The fields of the context object are available in the context of the template to be rendered.

### Licensing

AGPL-3.0
