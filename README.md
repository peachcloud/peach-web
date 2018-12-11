# peach-web

## Web Admin Interface for PeachCloud

**peach-web** provides a web interface for monitoring and interacting with the PeachCloud device. This will allow administration of the single-board computer (ie. Raspberry Pi) running PeachCloud, as well as the ssb-server and related plugins.

Initial development will focus on administration of the device itself, with SSB-related administration being integrated at a later stage.

### Design Principles

Minimal, low-tech & pretty. Aim for HTML-only, accept JS grudgingly (only if strictly necessary).

### Basic Project Outline

1. Setup sub-project repo
2. Write introductory documentation
3. Setup dev-diary in the Scuttleverse
4. Brainstorm interface requirements (feature list)
 - Pi-related data and interactivity
 - SSB-related data and interactivity
5. Create UI spec document
 - Map features to views (text)
6. Sketch interface mockups
7. Generate UX flow diagrams
 - Iterate between steps 6 & 7
8. Setup dev environment
 - Simple web server to host static content (possibly [Rocket](https://rocket.rs/))
9. Begin coding views
 - Return to steps 6 & 7 where necessary
10. Move towards alpha before seeking external input / testers

### Relevant Links

**Cypherlinks (Scuttlebutt)**

 - Project inception: ButtCloud SSBC Grant Proposal
  - %HqwAsltORROCh4uyOq6iV+SsqU3OuNUevnq+5dwCqVI=.sha256
  - [ssb-web viewer](http://viewer.scuttlebot.io/%25HqwAsltORROCh4uyOq6iV%2BSsqU3OuNUevnq%2B5dwCqVI%3D.sha256)
 - Project reconception: PeachCloud as a hardware product
  - %9NCyTf+oBxG0APlXRCKtrGZj3t+i+Kp3pKPN1gtFX2c=.sha256
  - [ssb-web viewer](http://viewer.scuttlebot.io/%259NCyTf%2BoBxG0APlXRCKtrGZj3t%2Bi%2BKp3pKPN1gtFX2c%3D.sha256)

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

TBC...
