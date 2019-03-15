window.onload = function () {
  $.get("/ip", function(data, status){
    console.log(data);
    //var ap0_addr = document.getElementById("ap0_addr");
    //var wlan0_addr = document.getElementById("wlan0_addr");
    var ap0 = document.querySelector('#ap0_addr');
    ap0.innerText = "ap0: " + data.data.ap0;
    var wlan0 = document.querySelector('#wlan0_addr');
    wlan0.innerText = "wlan0: " + data.data.wlan0;
  });

  $.get("/ssid", function(data, status){
    console.log(data);
    var ssid = document.querySelector('#ssid_name');
    ssid.innerText = "SSID: " + data.data;
  });
};
