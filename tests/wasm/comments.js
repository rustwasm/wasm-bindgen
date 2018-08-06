const fs = require('fs');
const assert = require('assert');

exports.assert_comments_exist = function() {
  const bindings_file = require.resolve('wasm-bindgen-test');
  const contents = fs.readFileSync(bindings_file);
  assert.ok(contents.includes("* annotated function"));
  assert.ok(contents.includes("* annotated struct type"));
  assert.ok(contents.includes("* annotated struct field"));
  assert.ok(contents.includes("* annotated struct method"));
};
