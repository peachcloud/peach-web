/*

behavioural layer for the `shutdown.html.tera` template,
corresponding to the web route `/shutdown`

 - intercept button clicks for reboot & shutdown
 - perform json api calls
 - update the dom

methods:

 PEACH_DEVICE.reboot();
 PEACH_DEVICE.shutdown();
 PEACH_DEVICE.flashMsg(status, msg);

*/

var PEACH_DEVICE = {};

// catch click of 'Reboot' button and make POST request
PEACH_DEVICE.reboot = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var rebootDevice = document.getElementById('rebootBtn');
        if (rebootDevice) {
            rebootDevice.addEventListener('click', function(e) {
                // prevent redirect on button press (default behavior)
                e.preventDefault();
                // write reboot flash message
                PEACH_DEVICE.flashMsg("success", "Rebooting the device...");
                // send reboot_device POST request
                fetch("/api/v1/device/reboot", {
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
                    PEACH_DEVICE.flashMsg(jsonData.status, jsonData.msg);
                })
            }, false);
        }
    });
}

// catch click of 'Shutdown' button and make POST request
PEACH_DEVICE.shutdown = function() {
    document.addEventListener('DOMContentLoaded', function() {
        var shutdownDevice = document.getElementById('shutdownBtn');
        if (shutdownDevice) {
            shutdownDevice.addEventListener('click', function(e) {
                // prevent form submission (default behavior)
                e.preventDefault();
                // write shutdown flash message
                PEACH_DEVICE.flashMsg("success", "Shutting down the device...");
                // send shutdown_device POST request
                fetch("/api/v1/device/shutdown", {
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
    } else if (status === "info") {
        elementClass = "center-text flash-message font-info";
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

var deviceInstance = PEACH_DEVICE;
deviceInstance.reboot();
deviceInstance.shutdown();
