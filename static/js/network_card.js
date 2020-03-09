/*

behavioural layer for the `network_card.html.tera` template,
corresponding to the web route `/network`

 - intercept form submissions
 - perform json api calls
 - update the dom

methods:

 PEACH_NETWORK.activateAp();
 PEACH_NETWORK.activateClient();
 PEACH_NETWORK.apOnline();
 PEACH_NETWORK.clientOffline();
 PEACH_NETWORK.clientOnline();
 PEACH_NETWORK.flashMsg(status, msg);

*/

var PEACH_NETWORK = {};

// catch click of 'Deploy Access Point' and make POST request
PEACH_NETWORK.activateAp = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var deployAP = document.getElementById('deployAccessPoint');
        if (deployAP) {
            deployAP.addEventListener('click', function(e) {
                // prevent form submission (default behavior)
                e.preventDefault();
                // send activate_ap POST request
                fetch("/api/v1/network/activate_ap", {
                    method: "post",
                    headers: {
                        'Accept': 'application/json',
                        'Content-Type': 'application/json'
                    },
                })
                .then( (response) => {
                    return response.json()
                })
                .then( (jsonData) => {
                    console.log(jsonData.msg);
                    // write json response message to ui
                    PEACH_NETWORK.flashMsg(jsonData.status, jsonData.msg);
                    // if ap activation is successful, update the ui
                    if (jsonData.status === "success") {
                        PEACH_NETWORK.apOnline();
                    }
                })
            });
        }
    });
}

// catch click of 'Enable WiFi' and make POST request
PEACH_NETWORK.activateClient = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var enableWifi = document.getElementById('connectWifi');
        if (enableWifi) {
            enableWifi.addEventListener('click', function(e) {
                // prevent form submission (default behavior)
                e.preventDefault();
                // send activate_ap POST request
                fetch("/api/v1/network/activate_client", {
                    method: "post",
                    headers: {
                        'Accept': 'application/json',
                        'Content-Type': 'application/json'
                    },
                })
                .then( (response) => {
                    return response.json()
                })
                .then( (jsonData) => {
                    console.log(jsonData.msg);
                    // write json response message to ui
                    PEACH_NETWORK.flashMsg(jsonData.status, jsonData.msg);
                    // if client activation is successful, update the ui
                    if (jsonData.status === "success") {
                        PEACH_NETWORK.clientOnline();
                    }
                })
            });
        }
    });
}

// update ui for access point mode (status: online)
PEACH_NETWORK.apOnline = function() {
    console.log('Activating AP Mode');

    // update network mode and status (icon & label)
    let i = document.getElementById("netModeIcon");
    i.className = "center icon icon-active";
    i.src = "icons/router.svg";
    let l = document.getElementById("netModeLabel");
    l.textContent = "ONLINE";

    // create Enable WiFi button and add it to button div
    var wifiButton = document.createElement("A");
    wifiButton.className = "button center";
    wifiButton.href = "/network/wifi/activate";
    wifiButton.id = "connectWifi";
    var label = "Enable WiFi";
    var buttonText = document.createTextNode(label);
    wifiButton.appendChild(buttonText);

    // append the new button to the buttons div
    let buttons = document.getElementById("buttons");
    buttons.appendChild(wifiButton);

    // remove the old 'Activate Access Point' button
    let apButton = document.getElementById("deployAccessPoint");
    apButton.style = "display: none;";
}

// update ui for wifi client mode (status: online)
PEACH_NETWORK.clientOnline = function() {
    console.log('Activating Client Mode');

    // update network mode and status (icon & label)
    let i = document.getElementById("netModeIcon");
    i.className = "center icon icon-active";
    i.src = "icons/wifi.svg";
    let l = document.getElementById("netModeLabel");
    l.textContent = "ONLINE";

    // TODO: think about updates for buttons (transition from ap mode)
}

// update ui for wifi client mode (status: offline)
PEACH_NETWORK.clientOffline = function() {
    console.log('Activating Client Mode');

    // update network mode and status (icon & label)
    let i = document.getElementById("netModeIcon");
    i.className = "center icon icon-inactive";
    i.src = "icons/wifi.svg";
    let l = document.getElementById("netModeLabel");
    l.textContent = "OFFLINE";

    // TODO: think about updates for buttons (transition from ap mode)
}

// display a message by appending a paragraph element
PEACH_NETWORK.flashMsg = function(status, msg) {
    // set the class of the element according to status
    var elementClass;
    if (status === "success") {
        elementClass = "center-text flash-message font-success";
    } else {
        elementClass = "center-text flash-message font-failure";
    };

    var flashElement = document.getElementById("flashMsg");
    // if flashElement exists, update the class & text
    if (flashElement) {
        flashElement.className = elementClass;
        flashElement.innerText = msg;
    // if flashElement does not exist, create it, set id, class, text & append
    } else {
        // create new div for flash message
        var flashDiv = document.createElement("DIV");
        // set div attributes
        flashDiv.id = "flashMsg";
        flashDiv.className = elementClass;
        // add json response message to flash message div
        var flashMsg = document.createTextNode(msg);
        flashDiv.appendChild(flashMsg);
        // insert the flash message div above the three icon grid div
        var gridDiv = document.getElementById("gridDiv");
        gridDiv.parentNode.insertBefore(flashDiv, gridDiv);
    }
}

var networkInstance = PEACH_NETWORK;

/* networkInstance.activateAp(); */

/*

async function exampleFetch() {
    const response = await fetch('/api/v1/network/state');
    const myJson = await response.json();
    //const jsonData = JSON.parse(myJson);
    console.log(myJson.data.wlan0);
    //var state = document.createElement("P");
    //state.innerText = ""jsonData.wlan0;
    //document.body.appendChild(state);
}

exampleFetch()

*/
