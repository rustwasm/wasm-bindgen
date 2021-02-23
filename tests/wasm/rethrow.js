const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.call_throw_one = function () {
  try {
    wasm.throw_one();
  } catch (e) {
    assert.strictEqual(e, 1);
  }
};

exports.call_ok = function () {
  wasm.nothrow();
};

exports.call_parse_or_throw_rust_error = function () {
  assert.strictEqual(wasm.parse_or_throw_rust_error("12"), 12);
  assert.throws(() => wasm.parse_or_throw_rust_error("12#"), new Error('invalid digit found in string'));
};