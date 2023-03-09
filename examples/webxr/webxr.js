import * as wasm from "./pkg";

var xrApp = new wasm.XrApp();
xrApp.init()
    .then(res => {
        console.log(res);
        xrApp.start();
    });