const assert = require('assert');

global.assert_dict_c = function(c) {
  assert.strictEqual(c.a, 1);
  assert.strictEqual(c.b, 2);
  assert.strictEqual(c.c, 3);
  assert.strictEqual(c.d, 4);
  assert.strictEqual(c.e, 5);
  assert.strictEqual(c.f, 6);
  assert.strictEqual(c.g, 7);
  assert.strictEqual(c.h, 8);
};

global.mk_dict_a = function() {
  return {};
};

global.assert_dict_required = function(c) {
  assert.strictEqual(c.a, 3);
  assert.strictEqual(c.b, "a");
  assert.strictEqual(c.c, 4);
};

global.assert_camel_case = function(dict) {
  assert.strictEqual(dict.wierd_fieldName, 1);
}
