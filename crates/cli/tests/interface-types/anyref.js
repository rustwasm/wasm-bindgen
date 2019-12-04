const assert = require('assert');
const wasm = require('wasm');

const obj = {};
assert.strictEqual(wasm.foo(obj), obj);

wasm.store('x');
assert.strictEqual(wasm.load(), 'x');

const obj2 = {};
wasm.store(obj2);
assert.strictEqual(wasm.load(), obj2);
assert.strictEqual(wasm.load(), obj2);

wasm.store(undefined);
assert.strictEqual(wasm.load(), undefined);
