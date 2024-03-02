const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_identity = a => a;
exports.js_option_identity = a => a;

exports.js_works = () => {
    assert.strictEqual(wasm.letter(), 'a');
    assert.strictEqual(wasm.face(), '😀');
    assert.strictEqual(wasm.rust_identity('Ղ'), 'Ղ');
    assert.strictEqual(wasm.rust_identity('ҝ'), 'ҝ');
    assert.strictEqual(wasm.rust_identity('Δ'), 'Δ');
    assert.strictEqual(wasm.rust_identity('䉨'), '䉨');
    assert.strictEqual(wasm.rust_js_identity('a'), 'a');
    assert.strictEqual(wasm.rust_js_identity('㊻'), '㊻');
    wasm.rust_letter('a');
    wasm.rust_face('😀');

    assert.throws(() => wasm.rust_js_identity('\uD83D'), /expected a number argument that is a valid Unicode scalar value, found 55357/);
    assert.throws(() => wasm.rust_js_option_identity('\uD83D'), /expected a number argument that is a valid Unicode scalar value, found 55357/);
};
