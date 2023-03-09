const assert = require('assert');
const wasm = require('wasm');

wasm.nop();
assert.strictEqual(wasm.roundtrip(1), 1);
