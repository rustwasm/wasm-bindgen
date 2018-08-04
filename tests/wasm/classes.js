const assert = require('assert');
const wasm = require('wasm-bindgen-test.js');

exports.test_simple = function() {
  const r = wasm.ClassesSimple.new();
  assert.strictEqual(r.add(0), 0);
  assert.strictEqual(r.add(1), 1);
  assert.strictEqual(r.add(1), 2);
  r.add(2);
  assert.strictEqual(r.consume(), 4);
  assert.throws(() => r.free(), /null pointer passed to rust/);

  const r2 = wasm.ClassesSimple.with_contents(10);
  assert.strictEqual(r2.add(1), 11);
  assert.strictEqual(r2.add(2), 13);
  assert.strictEqual(r2.add(3), 16);
  r2.free();

  const r3 = new wasm.ClassesSimple();
  assert.strictEqual(r3.add(42), 42);
  r3.free();
};

exports.test_strings = function() {
  const r = wasm.ClassesStrings1.new();
  r.set(3);
  let bar = r.bar('baz');
  r.free();
  assert.strictEqual(bar.name(), "foo-baz-3");
  bar.free();
};

exports.test_exceptions = function() {
  assert.throws(() => new wasm.ClassesExceptions1(), /cannot invoke `new` directly/);
  let a = wasm.ClassesExceptions1.new();
  a.free();
  assert.throws(() => a.free(), /null pointer passed to rust/);

  let b = wasm.ClassesExceptions1.new();
  b.foo(b);
  assert.throws(() => b.bar(b), /recursive use of an object/);

  let c = wasm.ClassesExceptions1.new();
  let d = wasm.ClassesExceptions2.new();
  assert.throws(() => c.foo(d), /expected instance of ClassesExceptions1/);
  d.free();
  c.free();
};

exports.test_pass_one_to_another = function() {
  let a = wasm.ClassesPassA.new();
  let b = wasm.ClassesPassB.new();
  a.foo(b);
  a.bar(b);
  a.free();
};

exports.take_class = function(foo) {
  assert.strictEqual(foo.inner(), 13);
  foo.free();
  assert.throws(() => foo.free(), /null pointer passed to rust/);
};

exports.test_constructors = function() {
  const foo = new wasm.ConstructorsFoo(1);
  assert.strictEqual(foo.get_number(), 1);
  foo.free();

  const foo2 = wasm.ConstructorsFoo.new(2);
  assert.strictEqual(foo2.get_number(), 2);
  foo2.free();

  const bar = new wasm.ConstructorsBar(3, 4);
  assert.strictEqual(bar.get_sum(), 7);
  bar.free();

  const bar2 = wasm.ConstructorsBar.other_name(5, 6);
  assert.strictEqual(bar2.get_sum(), 11);
  bar2.free();

  assert.strictEqual(wasm.cross_item_construction().get_sum(), 15);
};

exports.test_empty_structs = function() {
  wasm.OtherEmpty.return_a_value();
};

exports.test_public_fields = function() {
  const a = wasm.PublicFields.new();
  assert.strictEqual(a.a, 0);
  a.a = 3;
  assert.strictEqual(a.a, 3);

  assert.strictEqual(a.b, 0);
  a.b = 7;
  assert.strictEqual(a.b, 7);

  assert.strictEqual(a.c, 0);
  a.c = 8;
  assert.strictEqual(a.c, 8);

  assert.strictEqual(a.d, 0);
  a.d = 3.3;
  assert.strictEqual(a.d, 3);
};

exports.test_using_self = function() {
  wasm.UseSelf.new().free();
};

exports.test_readonly_fields = function() {
  const a = wasm.Readonly.new();
  assert.strictEqual(a.a, 0);
  a.a = 3;
  assert.strictEqual(a.a, 0);
  a.free();
};

exports.test_double_consume = function() {
  const r = wasm.DoubleConsume.new();
  assert.throws(() => r.consume(r), /Attempt to use a moved value/);
}


exports.test_js_rename = function() {
  wasm.JsRename.new().bar();
  wasm.classes_foo();
};
