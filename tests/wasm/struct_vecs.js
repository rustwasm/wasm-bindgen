const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.pass_struct_vec = () => {
    const el1 = new wasm.ArrayElement();
    const el2 = new wasm.ArrayElement();
    wasm.consume_struct_vec([el1, el2]);
};

exports.pass_invalid_struct_vec = () => {
    try {
        wasm.consume_struct_vec(['not a struct']);
    } catch (e) {
        assert.match(e.message, /array contains a value of the wrong type/)
        assert.match(e.stack, /consume_struct_vec/)
    }
};
