/*

behavioural layer for the `network_usage.html.tera` template,
corresponding to the web route `/network/wifi/usage`

 - intercept form submissions
 - perform json api calls
 - update the dom

methods:

 PEACH_NETWORK.updateAlerts();
 PEACH_NETWORK.toggleRxWarning();
 PEACH_NETWORK.toggleRxCutoff();
 PEACH_NETWORK.toggleTxWarning();
 PEACH_NETWORK.toggleTxCutoff();
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
            let rx_warn = formElement.elements.rx_warn.value;
            let rx_cut = formElement.elements.rx_cut.value;
            let tx_warn = formElement.elements.tx_warn.value;
            let tx_cut = formElement.elements.tx_cut.value;
            let rx_warn_flag = formElement.elements.rx_warn_flag.checked;
            let rx_cut_flag = formElement.elements.rx_cut_flag.checked;
            let tx_warn_flag = formElement.elements.tx_warn_flag.checked;
            let tx_cut_flag = formElement.elements.tx_cut_flag.checked;
            // perform json serialization
            var jsonData = JSON.stringify({
                "rx_warn": parseFloat(rx_warn),
                "rx_cut": parseFloat(rx_cut),
                "tx_warn": parseFloat(tx_warn),
                "tx_cut": parseFloat(tx_cut),
                "rx_warn_flag": rx_warn_flag,
                "rx_cut_flag": rx_cut_flag,
                "tx_warn_flag": tx_warn_flag,
                "tx_cut_flag": tx_cut_flag,
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

// update ui for rx warning
PEACH_NETWORK.toggleRxWarning = function() {
    document.addEventListener('DOMContentLoaded', function() {
        let i = document.getElementById("rxWarnIcon");
        let rxWarnCheck = document.getElementById("downWarnCheck");
        rxWarnCheck.addEventListener('click', function(e) {
            console.log('Toggling download warning icon state');
            if (rxWarnCheck.checked) {
                i.className = "icon";
            } else {
                i.className = "icon icon-inactive";
            }
        });
    });
};

// update ui for rx cutoff
PEACH_NETWORK.toggleRxCutoff = function() {
    document.addEventListener('DOMContentLoaded', function() {
        let i = document.getElementById("rxCutIcon");
        let rxCutCheck = document.getElementById("downCutCheck");
        rxCutCheck.addEventListener('click', function(e) {
            console.log('Toggling download cutoff icon state');
            if (rxCutCheck.checked) {
                i.className = "icon";
            } else {
                i.className = "icon icon-inactive";
            }
        });
    });
};

// update ui for tx warning
PEACH_NETWORK.toggleTxWarning = function() {
    document.addEventListener('DOMContentLoaded', function() {
        let i = document.getElementById("txWarnIcon");
        let txWarnCheck = document.getElementById("upWarnCheck");
        txWarnCheck.addEventListener('click', function(e) {
            console.log('Toggling upload warning icon state');
            if (txWarnCheck.checked) {
                i.className = "icon";
            } else {
                i.className = "icon icon-inactive";
            }
        });
    });
};

// update ui for tx cutoff
PEACH_NETWORK.toggleTxCutoff = function() {
    document.addEventListener('DOMContentLoaded', function() {
        let i = document.getElementById("txCutIcon");
        let txCutCheck = document.getElementById("upCutCheck");
        txCutCheck.addEventListener('click', function(e) {
            console.log('Toggling upload cutoff icon state');
            if (txCutCheck.checked) {
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
        buttonDiv.parentNode.insertBefore(flashDiv, buttonDiv.nextSibling);
    }
}

var usageInstance = PEACH_NETWORK;
usageInstance.toggleRxWarning();
usageInstance.toggleRxCutoff();
usageInstance.toggleTxWarning();
usageInstance.toggleTxCutoff();
usageInstance.updateAlerts();
