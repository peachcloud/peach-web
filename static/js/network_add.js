/*

behavioural layer for the `network_add.html.tera` template,
corresponding to the web route `/network/wifi/add`

 - intercept button click for add (form submission of credentials)
 - perform json api call
 - update the dom

methods:

 PEACH_DEVICE.add();
 PEACH_DEVICE.flashMsg(status, msg);

*/

var PEACH_DEVICE = {};

// catch click of 'Add' button and make POST request
PEACH_DEVICE.add = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var addWifi = document.getElementById('addWifi');
        if (addWifi) {
            addWifi.addEventListener('click', function(e) {
                // prevent redirect on button press (default behavior)
                e.preventDefault();
                // capture the ssid value
                var ssid = $("#ssid").val();
                // capture the password value
                var pass = $("#pass").val();
                // send add_wifi POST request
                fetch("/api/v1/network/wifi", {
                    method: "post",
                    headers: {
                        'Accept': 'application/json',
                        'Content-Type': 'application/json'
                    },
                    // serialize the JSON body
                    body: JSON.stringify({
                        ssid: ssid,
                        pass: pass
                    })
                })
                .then( (response) => {
                    return response.json()
                })
                .then( (jsonData) => {
                    console.log(jsonData.msg);
                    // write json response message to ui
                    PEACH_DEVICE.flashMsg(jsonData.status, jsonData.msg);
                })
            }, false);
        }
    });
}

// display a message by appending a paragraph element
PEACH_DEVICE.flashMsg = function(status, msg) {
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

var addInstance = PEACH_DEVICE;
addInstance.add();
