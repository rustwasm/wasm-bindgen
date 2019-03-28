import("./pkg/websocket")
.then(rust_modules => {

let ws = null;

document.forms[0].onsubmit = (e) => {
  e.preventDefault();
  if (ws !== null) {
    ws.close();
  }
  ws = new rust_modules.WebSocketFactory(document.getElementsByName("ws_server_url")[0].value);
  document.getElementById("status").innerText="Connected!";
  ws.init((data) => {
    console.log(data);
    let li = document.createElement('li');
    li.innerText = data.data;
    document.getElementById("ws_receives").appendChild(li);
  });

  document.forms[1].onsubmit = (e) => {
    e.preventDefault();
    ws.send(document.getElementsByName("ws_send_data")[0].value);
  }
}
});
