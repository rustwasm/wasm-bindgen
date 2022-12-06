const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_transparent_u32 = () => {
  const r = wasm.make_transparent_u32(42);
  assert.strictEqual(r, 42);
  assert.strictEqual(wasm.get_transparent_u32(r), 42);
  assert.strictEqual(r + 1, 43);
  assert.strictEqual(wasm.get_transparent_u32(r) + 1, 43);
  assert.strictEqual(typeof r, "number");
};

exports.js_transparent_u32_empty = () => {
  const r = wasm.make_transparent_u32_empty(42);
  assert.strictEqual(r, 42);
  assert.strictEqual(wasm.get_transparent_u32_empty(r), 42);
  assert.strictEqual(r + 1, 43);
  assert.strictEqual(wasm.get_transparent_u32_empty(r) + 1, 43);
  assert.strictEqual(typeof r, "number");
};

exports.js_trans_trans_u32 = () => {
  const r = wasm.make_trans_trans_u32(42);
  assert.strictEqual(r, 42);
  assert.strictEqual(wasm.get_trans_trans_u32(r), 42);
  assert.strictEqual(r + 1, 43);
  assert.strictEqual(wasm.get_trans_trans_u32(r) + 1, 43);
  assert.strictEqual(typeof r, "number");
};
