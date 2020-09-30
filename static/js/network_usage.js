/*

behavioural layer for the `network_usage.html.tera` template,
corresponding to the web route `/network/wifi/usage`

 - intercept form submissions
 - perform json api calls
 - update the dom

methods:

 PEACH_NETWORK.updateAlerts();
 PEACH_NETWORK.toggleWarning();
 PEACH_NETWORK.toggleCutoff();
 PEACH_NETWORK.flashMsg(status, msg);

*/

var PEACH_NETWORK = {};

// catch click of 'Update' and make POST request
PEACH_NETWORK.updateAlerts = function() {
    document.addEventListener('DOMContentLoaded', function() {
        document.body.addEventListener('submit', function(e) {
            // prevent redirect on button press (default behavior)
            e.preventDefault();
            // capture form data
            var formElement = document.querySelector("form");
            let warn = formElement.elements.warn.value;
            let cut = formElement.elements.cut.value;
            let warn_flag = formElement.elements.warn_flag.checked;
            let cut_flag = formElement.elements.cut_flag.checked;
            // perform json serialization
            var jsonData = JSON.stringify({
                "warn": parseFloat(warn),
                "cut": parseFloat(cut),
                "warn_flag": warn_flag,
                "cut_flag": cut_flag,
            });
            // write in-progress status message to ui
            PEACH_NETWORK.flashMsg("info", "Updating alert settings...");
            // send update_alerts POST request
            fetch("/api/v1/network/wifi/usage", {
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
}

// update ui for warning
PEACH_NETWORK.toggleWarning = function() {
    document.addEventListener('DOMContentLoaded', function() {
        let i = document.getElementById("warnIcon");
        let warnCheck = document.getElementById("warnCheck");
        warnCheck.addEventListener('click', function(e) {
            console.log('Toggling warning icon state');
            if (warnCheck.checked) {
                i.className = "icon";
            } else {
                i.className = "icon icon-inactive";
            }
        });
    });
};

// update ui for cutoff
PEACH_NETWORK.toggleCutoff = function() {
    document.addEventListener('DOMContentLoaded', function() {
        let i = document.getElementById("cutIcon");
        let cutCheck = document.getElementById("cutCheck");
        cutCheck.addEventListener('click', function(e) {
            console.log('Toggling cutoff icon state');
            if (cutCheck.checked) {
                i.className = "icon";
            } else {
                i.className = "icon icon-inactive";
            }
        });
    });
};

// display a message by appending a paragraph element
PEACH_NETWORK.flashMsg = function(status, msg) {
    // set the class of the element according to status
    var elementClass;
    if (status === "success") {
        elementClass = "capsule center-text flash-message font-success";
    } else if (status === "info") {
        elementClass = "capsule center-text flash-message font-info";
    } else {
        elementClass = "capsule center-text flash-message font-failure";
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
        buttonDiv.parentNode.insertBefore(flashDiv, buttonDiv.nextSibling);
    }
}

var usageInstance = PEACH_NETWORK;
usageInstance.toggleWarning();
usageInstance.toggleCutoff();
usageInstance.updateAlerts();
