const wasm = require('wasm-bindgen-test.js');

exports.js_auto_bind_math = () => {
    wasm.math(1.0, 2.0);
};
