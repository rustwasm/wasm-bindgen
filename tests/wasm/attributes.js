const wasm = require("wasm-bindgen-test.js");
const assert = require("assert");

exports.js_works = () => {
  assert.strictEqual(wasm.valid_export(), true);
  assert.strictEqual(wasm.invalid_export, undefined);
  assert.strictEqual(wasm.valid_attr_export(), true);
  assert.strictEqual(wasm.invalid_attr_export, undefined);
};
