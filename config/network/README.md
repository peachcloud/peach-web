## PeachCloud

### Network Configuration

hostapd and dnsmasq are used to run the device as an access point. wpa_supplicant is used to connect to WiFi access points (routers).

/usr/bin/createap (a bash script) is run as a systemd service on boot. This script creates the ap0 interface and sets a static IP.

/usr/bin/interface_checker (a bash script) is run every 5 minutes. It starts hostapd and dnsmasq if the wlan0 interface does not have an IP address (ie. is not connected to a router). If hostapd is running and the wlan0 interface has an IP address, hostapd and dnsmasq are stopped. If hostapd is running and the wlan0 interface does not have an IP address, the script does nothing.

### Setup

**Configure interfaces:**

sudo cp peach-web/config/network/interfaces /etc/network/interfaces

**Install wpa_supplicant and configure:**

sudo apt-get install wpa_supplicant  
sudo cp peach-web/config/network/wpa_supplicant.conf /etc/wpa_supplicant/wpa_supplicant.conf

**Install hostapd and configure:**

sudo apt-get install hostapd  
sudo cp peach-web/config/network/hostapd.conf /etc/hostapd/hostapd.conf

**Install dnsmasq and configure:**

sudo apt-get install dnsmasq  
sudo cp peach-web/config/network/dnsmasq.conf /etc/dnsmasq.conf

**Configure service for creating `ap0` interface on boot:**

sudo cp peach-web/config/network/createap.sh /usr/bin/createap  
sudo cp peach-web/config/network/create-ap.service /etc/systemd/system/  
sudo systemctl enable create-ap.service  

**Configure interface_checker to run every 5 minutes (as root):**

sudo cp peach-web/config/network/interface_checker.sh /usr/bin/interface_checker  
su  
crontab -e  
_Select a text editor_  
_Append the following line:_  
*/5 * * * * /usr/bin/interface_checker  
_Save and exit_  
