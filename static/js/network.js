/*
network.js

Contains all JavaScript interactions with the peach-network microservice
*/

// AP - Client Mode Switching
document.addEventListener('DOMContentLoaded', function(event) {
    var deployAP = document.getElementById('deployAccessPoint');
    if (deployAP) {
        deployAP.addEventListener('click', function() {
            console.log('Activating AP Mode');
            // update network mode and status (icon & label)
            let i = document.getElementById("netModeIcon");
            i.className = "center icon icon-inactive";
            i.src = "icons/router.svg";
            let l = document.getElementById("netModeLabel");
            l.textContent = "OFFLINE";
            // send activate_ap POST request
            fetch("/api/v1/network/activate_ap", {
                method: "post",
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
            })
            .then( (response) => {
                console.log(response);
            });
        });
    };

    var connectWifi = document.getElementById('connectWifi');
    if (connectWifi) {
        connectWifi.addEventListener('click', function() {
            console.log('Activating WiFi Client Mode');
            // update network mode and status (icon & label)
            let i = document.getElementById("netModeIcon");
            i.className = "center icon icon-inactive";
            i.src = "icons/wifi.svg";
            let l = document.getElementById("netModeLabel");
            l.textContent = "OFFLINE";
         // send activate_client POST request
            fetch("/api/v1/network/activate_client", {
                method: "post",
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
            })
            .then( (response) => {
                console.log(response);
            });
        });
    };
});
