/*
network.js

Contains all JavaScript interactions with the peach-network microservice
*/

// AP - Client Mode Switching
$(document).ready(function() {
    $('#deployAccessPoint').click(function() {
        console.log('Activating AP Mode');
        fetch("/api/activate_ap", {
            method: "post",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
        })
        .then( (response) => {
            console.log(response);i
            let i = document.getElementById("netModeIcon");
            i.src = "icons/router.svg";
            i.className = "center icon-inactive";
            i.value = "OFFLINE";
        });
    });

    $('#connectWifi').click(function() {
        console.log('Activating WiFi Client Mode');
        fetch("/api/activate_client", {
            method: "post",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
        })
        .then( (response) => {
            console.log(response);
            let i = document.getElementById("netModeIcon");
            i.src = "icons/wifi.svg";
            i.className = "center icon-inactive";
            i.value = "OFFLINE";
        });
    });
});
