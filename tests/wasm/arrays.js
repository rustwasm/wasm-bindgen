const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_ascending_array = () => {
    for (let a = 0; a < 33; a ++) {
        assert.strictEqual(
            wasm["asceding_array" + a](),
            [...Array(a).keys()]
        );
    }
};

exports.js_product = () => {
    let array = [];
    for (let a = 0; a < 33; a ++) {
        assert.strictEqual(
            wasm["product" + a](array),
            array.reduce((acc, value) => acc * value, 1)
        );
        array.push(a);
    }
};
