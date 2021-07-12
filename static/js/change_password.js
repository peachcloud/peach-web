/*
*   behavioural layer for the `change_password.html.tera` template,
 */

// catch click of 'Save' button and make POST request
PEACH.add = function() {
    document.addEventListener('DOMContentLoaded', function() {
        document.body.addEventListener('submit', function(e) {
            // prevent redirect on button press (default behavior)
            e.preventDefault();
            // capture form data
            var formElement = document.querySelector("form");
            // create form data object from the wifiCreds form element
            var formData = new FormData(formElement);
            var object = {};
            // assign values from form
            formData.forEach(function(value, key){
                object[key] = value;
            });
            // perform json serialization
            console.log(object);
            var jsonData = JSON.stringify(object);
            // write in-progress status message to ui
            PEACH.flashMsg("info", "Saving new password.");
            // send add_wifi POST request
            fetch("/api/v1/settings/change_password", {
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
                PEACH.flashMsg(jsonData.status, jsonData.msg);
            })
        }, false);
    });
}

var addInstance = PEACH;
addInstance.add();
