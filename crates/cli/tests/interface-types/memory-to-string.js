const wasm = require('wasm');
const assert = require('assert');

assert.strictEqual(wasm.foo(), 'foo');
assert.strictEqual(wasm.hexa(), 'hexa');
