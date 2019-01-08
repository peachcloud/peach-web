#!/bin/bash
ifdown wlan0
iw dev wlan0 interface add ap0 type __ap
ip address add 11.11.11.10/24 brd + dev ap0
systemctl start hostapd.service
systemctl start dnsmasq.service
