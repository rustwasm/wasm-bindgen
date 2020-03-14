import * as wasm from "webxr";

// wasm.run_webxr().then(function(res) {
//     console.log("Done")
// });

var xrApp = new wasm.XrApp();
xrApp.init()
    .then(res => {
        console.log(res);
        xrApp.start();
    });