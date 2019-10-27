/*
network.js

Contains all JavaScript interactions with the peach-network microservice
*/

// AP - Client Mode Switching
$(document).ready(function() {
    $('#wifiToggle').change(function() {
        var statusIcon = document.getElementById("statusIcon");
        if(this.checked) {
            document.getElementById("routerIcon").className = "icon-active switch-icon-right";
            document.getElementById("wifiIcon").className = "icon-inactive switch-icon-left";
            statusIcon.src = "icons/router.svg";
            $.post("/api/activate_ap",
                function() {
                    console.log('Activating AP Mode');
                }
            );
        } else {
            document.getElementById("wifiIcon").className = "icon-active switch-icon-left";
            document.getElementById("routerIcon").className = "icon-inactive switch-icon-right";
            statusIcon.src = "icons/wifi.svg";
            $.post("/api/activate_client",
                function() {
                    console.log('Activating Client Mode');
                }
            );
        }
    })
})
