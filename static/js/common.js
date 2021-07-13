/*
*
*   Common javascript functions shared by multiple pages:
*   - flashMsg
*   - logout
*
*/

var PEACH = {};

// display a message by appending a paragraph element
PEACH.flashMsg = function(status, msg) {
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

// add click event to logout button which logs out of http basic auth
// by "trying to login" with invalid credentials (user@logout)
document.getElementById('logoutButton').onclick = function(e){
  e.preventDefault();
  var logoutUrl =  "http://user:logout@" + window.location.hostname
  window.location = logoutUrl;
}

var addInstance = PEACH;
addInstance.add();
addInstance.logout();
