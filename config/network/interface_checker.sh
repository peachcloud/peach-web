#!/bin/bash
#
# Interface checker
# Checks to see whether interface has an IP address, if it doesn't assume it's down and start hostapd
# Based on original script by SirLagz
#
Interface='wlan0'
HostAPDIP='11.11.11.10'
echo "-----------------------------------"
echo "Checking connectivity of $Interface"
NetworkUp=`ip addr show $Interface`
IP=`echo "$NetworkUp" | grep inet | wc -l`
if [[ $IP -eq 0 ]]; then
  echo "Connection is down"

hostapd=`pidof hostapd`
if [[ -z $hostapd ]]; then
  echo "Attempting to start hostapd"
  systemctl start hostapd.service
  echo "Attempting to start dnsmasq"
  systemctl start dnsmasq.service
fi
elif [[ $IP -gt 0 && $NetworkUp =~ $HostAPDIP ]]; then
  echo "IP is $HostAPDIP - hostapd is running"
else
  echo "Connection is up"
  hostapd=`pidof hostapd`
  if [[ ! -z $hostapd ]]; then
    echo "Attempting to stop hostapd"
    systemctl stop hostapd.service
    echo "Attempting to stop dnsmasq"
    systemctl stop dnsmasq.service
  fi
fi
echo "-----------------------------------"
