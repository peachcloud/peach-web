# peach-web

## Web Admin Interface for PeachCloud

**peach-web** provides a web interface for monitoring and interacting with the PeachCloud device. This will allow administration of the single-board computer (ie. Raspberry Pi) running PeachCloud, as well as the ssb-server and related plugins.

Initial development will focus on administration of the device itself, with SSB-related administration being integrated at a later stage.

_Note: This is a work-in-progress._

### Setup

Clone this repo:

`git clone https://github.com/peachcloud/peach-web.git`

Move into the repo and compile:

`cd peach-web`  
`cargo build`

Run the tests:

`cargo test`

Run the binary:

`./target/debug/peach-web`

_Note: Networking functionality requires peach-network microservice to be running._


### JSON API

| Endpoint | Method | Parameters | Description |
| --- | --- | --- | --- |
| /ip | GET | | Returns IP address values for wlan0 & ap0 interfaces |
| /ssid | GET | | Returns SSID for connected WiFi network |
| /wifi_credentials | POST | `ssid` & `pass` | Submit SSID & password to create new WiFi connection |

-----

### Relevant Links

**Cypherlinks (Scuttlebutt)**

 - Project inception: ButtCloud SSBC Grant Proposal
   - %HqwAsltORROCh4uyOq6iV+SsqU3OuNUevnq+5dwCqVI=.sha256
   - [ssb-web viewer](http://viewer.scuttlebot.io/%25HqwAsltORROCh4uyOq6iV%2BSsqU3OuNUevnq%2B5dwCqVI%3D.sha256)
 - Project reconception: PeachCloud as a hardware product
   - %9NCyTf+oBxG0APlXRCKtrGZj3t+i+Kp3pKPN1gtFX2c=.sha256
   - [ssb-web viewer](http://viewer.scuttlebot.io/%259NCyTf%2BoBxG0APlXRCKtrGZj3t%2Bi%2BKp3pKPN1gtFX2c%3D.sha256)
 - PeachCloud Web Interface: Dev Diary
   - %mKUByRp4Gib6fqP1q2/dHg+ueSoR+Sj2Y0D7T0Np0D4=.sha256
   - [ssb-web viewer](http://viewer.scuttlebot.io/%25mKUByRp4Gib6fqP1q2%2FdHg%2BueSoR%2BSj2Y0D7T0Np0D4%3D.sha256)

**Legacy Web Links**

 - [PeachCloud project meta information](http://peachcloud.org)

### Developer Contacts

**PeachCloud Project Lead**

@dinosaur

 - [Twitter](https://twitter.com/ahdinosaur)
 - [GitHub](https://github.com/ahdinosaur)
 - Scuttlebutt (@6ilZq3kN0F+dXFHAPjAwMm87JEb/VdB+LC9eIMW3sa0=.ed25519)

**PeachCloud Web Design**

@glyph

 - [GitHub](https://github.com/mycognosist)
 - Scuttlebutt (@HEqy940T6uB+T+d9Jaa58aNfRzLx9eRWqkZljBmnkmk=.ed25519)

### Licensing

AGPL-3.0
