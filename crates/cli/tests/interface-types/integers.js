const assert = require('assert');
const wasm = require('wasm');

assert.strictEqual(wasm.add_i8(0, 1), 1);
assert.strictEqual(wasm.add_u8(0, 1), 1);
assert.strictEqual(wasm.add_i16(0, 1), 1);
assert.strictEqual(wasm.add_u16(0, 1), 1);
assert.strictEqual(wasm.add_i32(0, 1), 1);
assert.strictEqual(wasm.add_u32(0, 1), 1);
