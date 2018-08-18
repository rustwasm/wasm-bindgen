const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.variadic_sum = () =>
    Array.from(arguments).reduce((acc, val) => acc + val, 0);

