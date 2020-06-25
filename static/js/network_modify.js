/*

behavioural layer for the `network_modify.html.tera` template

 - intercept button click for modify (form submission of credentials)
 - perform json api call
 - update the dom

methods:

 PEACH_NETWORK.modify();
 PEACH_NETWORK.flashMsg(status, msg);

*/

var PEACH_NETWORK = {};

// catch click of 'Save' button and make POST request
PEACH_NETWORK.modify = function() {
    document.addEventListener('DOMContentLoaded', function() {
        document.body.addEventListener('submit', function(e) {
            // prevent redirect on button press (default behavior)
            e.preventDefault();
            // capture form data
            var formElement = document.querySelector("form");
            // create form data object from the wifiModify form element
            var formData = new FormData(formElement);
            var object = {};
            // assign ssid and pass from form
            formData.forEach(function(value, key){
                object[key] = value;
            });
            // perform json serialization
            var jsonData = JSON.stringify(object);
            // write in-progress status message to ui
            PEACH_NETWORK.flashMsg("info", "Updating WiFi password...");
            // send new_password POST request
            fetch("/api/v1/network/wifi/modify", {
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

var modifyInstance = PEACH_NETWORK;
modifyInstance.modify();
