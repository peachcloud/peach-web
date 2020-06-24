/*

behavioural layer for the `network_detail.html.tera` template,
corresponding to the web route `/network/wifi?<ssid>`

 - intercept button clicks for connect, disconnect and forget
 - perform json api call
 - update the dom

methods:

 PEACH_NETWORK.connect();
 PEACH_NETWORK.disconnect();
 PEACH_NETWORK.forget();
 PEACH_NETWORK.flashMsg(status, msg);

*/

var PEACH_NETWORK = {};

// catch click of 'Connect' button (form) and make POST request
PEACH_NETWORK.connect = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var connectWifi = document.getElementById('connectWifi');
        if (connectWifi) {
            connectWifi.addEventListener('click', function(e) {
                // prevent form submission (default behavior)
                e.preventDefault();
                // retrieve ssid value and append to form data object
                var ssid = document.getElementById('connectSsid').value;
                // create empty form data object
                var formData = new FormData();
                // append the ssid value from the form
                formData.append("ssid", ssid);
                var object = {};
                // assign ssid from form
                formData.forEach(function(value, key){
                    object[key] = value;
                });
                // perform json serialization
                var jsonData = JSON.stringify(object);
                // send add_wifi POST request
                fetch("/api/v1/network/wifi/connect", {
                    method: "post",
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: jsonData
                })
                .then( (response) => {
                    return response.json()
                })
                .then( (jsonData) => {
                    // write json response message to ui
                    PEACH_NETWORK.flashMsg(jsonData.status, jsonData.msg);
                })
            }, false);
        });
    });
}

// catch click of 'Disconnect' button and make POST request
PEACH_NETWORK.disconnect = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var disconnectWifi = document.getElementById('wifiDisconnect');
        if (disconnectWifi) {
            disconnectWifi.addEventListener('click', function(e) {
                // prevent form submission (default behavior)
                e.preventDefault();
                // send disconnect_wifi POST request (no body data required)
                fetch("/api/v1/network/wifi/disconnect", {
                    method: "post",
                    headers: {
                        'Content-Type': 'application/json',
                    }
                })
                .then( (response) => {
                    return response.json()
                })
                .then( (jsonData) => {
                    // write json response message to ui
                    PEACH_NETWORK.flashMsg(jsonData.status, jsonData.msg);
                })
            }, false);
        });
    });
}

// catch click of 'Forget' button (form) and make POST request
PEACH_NETWORK.forget = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var forgetWifi = document.getElementById('forgetWifi');
        if (forgetWifi) {
            forgetWifi.addEventListener('click', function(e) {
                // prevent form submission (default behavior)
                e.preventDefault();
                // retrieve ssid value and append to form data object
                var ssid = document.getElementById('forgetSsid').value;
                // create empty form data object
                var formData = new FormData();
                // append the ssid value from the form
                formData.append("ssid", ssid);
                var object = {};
                // assign ssid from form
                formData.forEach(function(value, key){
                    object[key] = value;
                });
                // perform json serialization
                var jsonData = JSON.stringify(object);
                // send forget_ap POST request
                fetch("/api/v1/network/wifi/forget", {
                    method: "post",
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: jsonData
                })
                .then( (response) => {
                    return response.json()
                })
                .then( (jsonData) => {
                    // write json response message to ui
                    PEACH_NETWORK.flashMsg(jsonData.status, jsonData.msg);
                })
            }, false);
        });
    });
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
        // insert the flash message div below the button div
        var buttonDiv = document.getElementById("buttonDiv");
        // flashDiv will be added to the end since buttonDiv is the last
        // child within the parent element (card-container div)
        buttonDiv.parentNode.insertBefore(flashDiv, buttonDiv.nextSibling);
    }
}

var detailInstance = PEACH_NETWORK;
detailInstance.connect();
detailInstance.disconnect();
detailInstance.forget();
