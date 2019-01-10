## PeachCloud

### Network Configuration

/usr/bin/createap (a bash script) is run as a systemd service on boot. This script creates the ap0 interface and sets a static IP.

/usr/bin/interface_checker is run every 5 minutes. It starts hostapd and dnsmasq if the wlan0 interface does not have an IP address (ie. is not connected to a router). If hostapd is running and the wlan0 interface has an IP address, hostapd and dnsmasq are stopped. If hostapd is running and the wlan0 interface does not have an IP address, the script does nothing.

### Setup

Configure service for creating `ap0` interface on boot:

sudo cp peach-web/config/network/createap.sh /usr/bin/createap
sudo cp peach-web/config/network/create-ap.service /etc/systemd/system/
sudo systemctl enable create-ap.service

Configure interface_checker to run every 5 minutes (as root):

sudo cp peach-web/config/network/interface_checker.sh /usr/bin/interface_checker
su
crontab -e
_Select a text editor_
_Append the following line: _
*/5 * * * * /usr/bin/interface_checker
_Save and exit_
