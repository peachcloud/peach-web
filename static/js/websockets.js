var socket = new WebSocket("ws://peach.local:2794", "rust-websocket");

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
