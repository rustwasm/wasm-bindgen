const wasm = require('wasm');
const assert = require('assert');

assert.strictEqual(wasm.foo(), 0);
assert.strictEqual(wasm.get(), 1);
