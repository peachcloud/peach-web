# peach-web

## Features List

A first draft of desirable features for the PeachCloud web admin interface.

_Note: This is a work-in-progress. Expect changes._

_First-pass at organizing features categorically_

**Summary:**

- Profile
- Peers
- Invites
- System Status
- Configuration
- Documentation
- Network
  - _Not sure if this one is made redundant by System Status & Configuration_
  - _Could also be 'Monitoring' (showing graphs of SSB network acitivty_

**Detail:**

- Profile
  - Display avatar
  - Set avatar (upload file)
  - Display bio
  - Update bio
- Peers
  - List friends
  - List followers
  - List follows
  - List locally-connected peers
  - Follow
  - Unfollow
  - Block
  - Mute (private block)
  - Private-message a peer
- Invites
  - Create an invite
    - Text-based (hash)
    - Image-based (QR-code)
    - Audio-based (possible?)
  - Share an invite
    - Send to a peer within SSB (private message)
    - Share publically within SSB (public post)
    - Send via email
  - Accept an invite
  - Monitor an invite
    - Check if the invite has been accepted
    - For multi-use invites, show number of used & unused invite-slots
  - Cancel an invite (_not sure if this is currently possible_)
- Documentation
  - Browse
    - Scuttlebot
    - Scuttlebutt
    - PeachCloud
    - _Link to external docs or host locally for offline-first viewing?_
  - Search
  - Notes
    - Add personal notes to document specific workflows etc.
    - Display notes
    - Delete notes
- System status
  - Hardware
    - CPU usage
    - Memory usage
    - Storage usage
    - Disk I/O
  - Software
    - Version info of PeachCloud, sbot, plugins
    - Scripts
    - Plugins
  - Power
    - Power source (mains, battery, solar panels)
      - Battery level and status (ie. 60% - charging)
  - Network
    - Display network mode (AP or client)
      - If AP, list connected devices
    - Display current connection(s)
      - Ethernet
      - WiFi
      - Bluetooth
      - NFC
      - LoRa
    - Display signal strength
    - Display bandwidth usage
    - Display hostname & external IP
    - Display internal IP
  - Logs
    - Display system logs
  - Errors
    - List errors
    - Report a bug / error
      - Via SSB message
      - Via email
- Configuration
  - Access control
    - Change user password
    - Change administrator password
    - Manage guest account
    - Enable / disable SSH
  - Blob management
    - Prune blobs
      - By size
      - By date
      - By author
  - Network
    - Set network mode (AP or client)
    - List available networks
    - Connect to a network
    - Disconnect from a network
    - Routing
      - Select routing over IPv4, IPv6
      - Enable / disable routing over CJDNS / Yggdrail
  - Updates
    - Check for available updates
    - Download updates
    - Install / apply update
  - Backups
    - Create backup
      - Secret key
      - Configuration (device settings)
    - Export backup
      - External storage (USB)
      - Dark Crystal
      - IPFS / Dat
    - List backup history
    - Schedule backups
    - Delete previous backups / backup history
  - Alerts
    - Set alerts based on CPU, memory, disk, bandwith-usage thresholds
    - List previously-defined alerts
    - Delete alerts
  - Plugins
    - List available plugins / extensions
      - _E.g. ssb-web viewer, git-ssb viewer_
    - Activate a plugin
    - Deactivate a plugin
    - Define plugin settings
  - Miscellaneous
    - List current datetime
    - Set datetime
    - Display current timezone
    - Set timezone
