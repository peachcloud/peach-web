/*

behavioural layer for the `configure_dns.html.tera` template,
corresponding to the web route `/network/dns`

 - intercept button click for add (form submission of credentials)
 - perform json api call
 - update the dom

methods:

 PEACH_DNS.configure();
 PEACH_DNS.flashMsg(status, msg);

*/

var PEACH_DNS = {};

// catch click of 'Add' button and make POST request
PEACH_DNS.add = function() {
    document.addEventListener('DOMContentLoaded', function() {
        document.body.addEventListener('submit', function(e) {
            // prevent redirect on button press (default behavior)
            e.preventDefault();
            // capture form data
            var formElement = document.querySelector("form");
            // create form data object from the wifiCreds form element
            var formData = new FormData(formElement);
            var object = {};
            // set checkbox to false (the value is only passed to formData if it is "on")
            object["enable_dyndns"] = false;
            // assign values from form
            formData.forEach(function(value, key){
                // convert checkbox to bool
                if (key === "enable_dyndns") {
                    value = (value === "on");
                }
                object[key] = value;
            });
            // perform json serialization
            console.log(object);
            var jsonData = JSON.stringify(object);
            // write in-progress status message to ui
            PEACH_DNS.flashMsg("info", "Saving new DNS configurations");
            // send add_wifi POST request
            fetch("/api/v1/dns/configure", {
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
                PEACH_DNS.flashMsg(jsonData.status, jsonData.msg);
            })
        }, false);
    });
}

// display a message by appending a paragraph element
PEACH_DNS.flashMsg = function(status, msg) {
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
        // flashDiv will be added to the end since buttonDiv is the last
        // child within the parent element (card-container div)
        buttonDiv.parentNode.insertBefore(flashDiv, buttonDiv.nextSibling);
    }
}

var addInstance = PEACH_DNS;
addInstance.add();
