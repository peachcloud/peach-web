$(document).on('submit', '#wifiCreds', function(event){
  event.preventDefault();
  var ssid = $("#ssid").val();
  var pass = $("#pass").val();
  $.post("/add_wifi",
  {
    ssid: ssid,
    pass: pass
  },
  function(data){
    console.log("Status: " + data.status + "\nData: " + data.msg);
  });
  this.reset();
});
