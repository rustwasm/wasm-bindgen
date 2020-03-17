import * as wasm from "webxr";

var xrApp = new wasm.XrApp();
xrApp.init()
    .then(res => {
        console.log(res);
        xrApp.start();
    });