document.addEventListener('DOMContentLoaded', function(event) {
    var addWifi = document.getElementById('addWifi');
    if (addWifi) {
        addWifi.addEventListener('click', function() {
            console.log('Uploading new WiFi credentials & attempting connection');
            var ssid = document.getElementById(ssid);
            var pass = document.getElementById(pass);
            const data = {
                ssid: ssid,
                pass: pass,
            };
            // send activate_ap POST request
            fetch('/api/v1/network/wifi', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(data)
            })
            .then(response => response.json())
            .then(response => console.log(response));
        });
    };
});
