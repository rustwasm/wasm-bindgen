const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_works = () => {
    assert(wasm.Testable.method);
    assert(wasm.Testable.second);
    assert.notDeepStrictEqual(wasm.Testable.method, wasm.Testable.second);
}
