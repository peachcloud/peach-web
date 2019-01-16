var socket = new WebSocket("ws://127.0.0.1:2794", "rust-websocket");

socket.onmessage = function (event) {
    var ip = document.getElementById("ip_addr");
    var text = document.createTextNode(event.data);
    ip.appendChild(text);
};

function send(element) {
    var input = document.getElementById(element);
    socket.send(input.value);
    input.value = "";
}
