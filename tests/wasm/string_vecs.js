const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.pass_string_vec = () => {
    wasm.consume_string_vec(["hello", "world"]);
};

exports.pass_invalid_string_vec = () => {
    try {
        wasm.consume_string_vec([42]);
    } catch (e) {
        assert.match(e.message, /array contains a value of the wrong type/)
        assert.match(e.stack, /consume_string_vec/)
    }
};
