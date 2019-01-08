#!/bin/bash
systemctl stop hostapd.service
systemctl stop dnsmasq.service
iw dev ap0 del
ifup wlan0
