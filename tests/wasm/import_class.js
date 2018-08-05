const assert = require('assert');
const wasm = require('wasm-bindgen-test');

exports.math_log = Math.log;

exports.StaticFunction = class {
  static bar() { return 2; }
};

class Construct {
  static create() {
    const ret = new Construct();
    ret.internal_string = 'this';
    return ret;
  }

  get_internal_string() {
    return this.internal_string;
  }

  append_to_internal_string(s) {
    this.internal_string += s;
  }

  assert_internal_string(s) {
    assert.strictEqual(this.internal_string, s);
  }
}

Construct.internal_string = '';
exports.Construct = Construct;

exports.NewConstructors = class {
  constructor(field) {
    this.field = field;
  }

  get() {
    return this.field + 1;
  }
};

let switch_called = false;
class SwitchMethods {
  constructor() {
  }

  static a() {
    switch_called = true;
  }

  b() {
    switch_called = true;
  }
}
exports.SwitchMethods = SwitchMethods;
exports.switch_methods_called = function() {
  const tmp = switch_called;
  switch_called = false;
  return tmp;
};
exports.switch_methods_a = function() { SwitchMethods.a = function() {}; };
exports.switch_methods_b = function() { SwitchMethods.prototype.b = function() {}; };

exports.Properties = class {
  constructor() {
    this.num = 1;
  }

  get a() {
    return this.num;
  }

  set a(val) {
    this.num = val;
  }
};

exports.RenameProperties = class {
  constructor() {
    this.num = 1;
  }

  get a() {
    return this.num;
  }

  set a(val) {
    this.num = val;
  }
};

class Options {
}
exports.Options = Options;

exports.take_none = function(val) {
  assert.strictEqual(val, undefined);
};

exports.take_some = function(val) {
  assert.strictEqual(val === undefined, false);
};

exports.return_null = function() {
  return null;
};

exports.return_undefined = function() {
  return undefined;
};

exports.return_some = function() {
  return new Options();
};

exports.run_rust_option_tests = function() {
  wasm.rust_take_none();
  wasm.rust_take_none(null)
  wasm.rust_take_none(undefined);
  wasm.rust_take_some(new Options());
  assert.strictEqual(wasm.rust_return_none(), undefined);
  assert.strictEqual(wasm.rust_return_none(), undefined);
  assert.strictEqual(wasm.rust_return_some() === undefined, false);
};
