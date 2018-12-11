# peach-web

## Features List

A first draft of desirable features for the PeachCloud web admin interface.

_Note: This is a work-in-progress. Expect changes._

### OS and Hardware-Related Features

**Data to be displayed**

- IP address (internal)
- Domain name (external)
- Monitoring (values and graphs)
  - System uptime
  - Core temperature
  - CPU usage
  - Disk usage
  - Memory usage
  - Bandwidth usage
  - File I/O
  - Power usage (voltage, current etc. - particularly useful if device is battery / solar / wind-powered)

**Commands**

- Shutdown device
- Reboot device
- Check for available software updates
- Update / upgrade software
- Backup data / config

### Scuttlebot-Related Features

**Data to be displayed**

- Pub / device name
- Profile image
- Profile bio
- Friends and follow(er)s (counts, lists or both)
- Version information (sbot & plugins)
- Installed plugins

**Commands**

- Generate invites (output as text and / or QR-code)
- Add / remove plugins
- Update device name, image and bio

**Other**

- Integration of ssb-web viewer and git-ssb viewer

### Other Features

May include other decentralized services (e.g. Dat or IPFS), routing (e.g. CJDNS, Yggdrasil, Tor) etc.
