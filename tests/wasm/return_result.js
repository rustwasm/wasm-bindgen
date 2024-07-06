const assert = require('assert');
const wasm = require('wasm-bindgen-test');

exports.call_exports = function () {
  assert.strictEqual(undefined, wasm.nothing());
  assert.strictEqual(3, wasm.return_3());
  assert.strictEqual(4, wasm.return_4());
  assert.strictEqual(42, wasm.return_option());
  assert.strictEqual(42, wasm.return_option_some());
  assert.strictEqual(undefined, wasm.return_option_none());
};

exports.return_value = function () {
  return 'ok';
};

exports.return_some = function () {
  return 'ok';
};

exports.return_none = function () {};
