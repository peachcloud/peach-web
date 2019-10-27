/*
network.js

Contains all JavaScript interactions with the peach-network microservice
*/

// AP - Client Mode Switching
$(document).ready(function() {
    $('#wifiToggle').change(function() {
        var statusIcon = document.getElementById("statusIcon");
        if(this.checked) {
            console.log('Activating AP Mode');
            document.getElementById("routerIcon").className = "icon-active switch-icon-right";
            document.getElementById("wifiIcon").className = "icon-inactive switch-icon-left";
            statusIcon.src = "icons/router.svg";
        } else {
            console.log('Activating Client Mode');
            document.getElementById("wifiIcon").className = "icon-active switch-icon-left";
            document.getElementById("routerIcon").className = "icon-inactive switch-icon-right";
            statusIcon.src = "icons/wifi.svg";
        }
    })
})
