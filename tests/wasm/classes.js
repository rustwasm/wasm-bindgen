const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_simple = () => {
    const r = new wasm.ClassesSimple(0);
    assert.strictEqual(r.add(0), 0);
    assert.strictEqual(r.add(1), 1);
    assert.strictEqual(r.add(1), 2);
    r.add(2);
    assert.strictEqual(wasm.ClassesSimple.consume(r), 4);
    // assert.throws(() => r[wasm.__wbg_free](), /null pointer passed to rust/);

    const r2 = new wasm.ClassesSimple(10);
    assert.strictEqual(r2.add(1), 11);
    assert.strictEqual(r2.add(2), 13);
    assert.strictEqual(r2.add(3), 16);
    r2[wasm.__wbg_free]();

    const r3 = new wasm.ClassesSimple(0);
    assert.strictEqual(r3.add(42), 42);
    r3[wasm.__wbg_free]();
};

exports.js_strings = () => {
    const r = new wasm.ClassesStrings1();
    r.set(3);
    let bar = r.bar('baz');
    r[wasm.__wbg_free]();
    assert.strictEqual(bar.name(), 'foo-baz-3');
    bar[wasm.__wbg_free]();
};

exports.js_exceptions = () => {
    // this test only works when `--debug` is passed to `wasm-bindgen` (or the
    // equivalent thereof)
    if (require('process').env.WASM_BINDGEN_NO_DEBUG)
        return;
    // assert.throws(() => new wasm.ClassesExceptions1(), /cannot invoke `new` directly/);
    let a = new wasm.ClassesExceptions1();
    a[wasm.__wbg_free]();
    assert.throws(() => a[wasm.__wbg_free](), /null pointer passed to rust/);

    let b = new wasm.ClassesExceptions1();
    b.foo(b);
    assert.throws(() => b.bar(b), /recursive use of an object/);

    let c = new wasm.ClassesExceptions1();
    let d = new wasm.ClassesExceptions2();
    assert.throws(() => c.foo(d), /expected instance of ClassesExceptions1/);
    d[wasm.__wbg_free]();
    c[wasm.__wbg_free]();
};

exports.js_pass_one_to_another = () => {
    let a = new wasm.ClassesPassA();
    let b = new wasm.ClassesPassB();
    a.foo(b);
    a.bar(b);
    a[wasm.__wbg_free]();
};

exports.take_class = foo => {
    assert.strictEqual(foo.inner(), 13);
    foo[wasm.__wbg_free]();
    assert.throws(() => foo[wasm.__wbg_free](), /null pointer passed to rust/);
};

exports.js_constructors = () => {
    const foo = new wasm.ConstructorsFoo(1);
    assert.strictEqual(foo.get_number(), 1);
    foo[wasm.__wbg_free]();

    if (!require('process').env.WASM_BINDGEN_NO_DEBUG)
        assert.throws(() => new wasm.ConstructorsBar());
    const foo2 = new wasm.ConstructorsFoo(2);
    assert.strictEqual(foo2.get_number(), 2);
    foo2[wasm.__wbg_free]();

    const bar = new wasm.ConstructorsBar(3, 4);
    assert.strictEqual(bar.get_sum(), 7);
    bar[wasm.__wbg_free]();

    assert.strictEqual(new wasm.ConstructorsBar(0, 0).other_name, undefined);
    const bar2 = new wasm.ConstructorsBar(5, 6);
    assert.strictEqual(bar2.get_sum(), 11);
    bar2[wasm.__wbg_free]();

    assert.strictEqual(wasm.cross_item_construction().get_sum(), 15);
};

exports.js_empty_structs = () => {
    wasm.OtherEmpty.return_a_value();
};

exports.js_public_fields = () => {
    const a = new wasm.PublicFields();
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

    assert.strictEqual(a.skipped, undefined);
};

exports.js_using_self = () => {
    (new wasm.UseSelf())[wasm.__wbg_free]();
};

exports.js_readonly_fields = () => {
    const a = new wasm.Readonly();
    assert.strictEqual(a.a, 0);
    a.a = 3;
    assert.strictEqual(a.a, 0);
    a[wasm.__wbg_free]();
};

exports.js_double_consume = () => {
    const r = new wasm.DoubleConsume();
    assert.throws(() => r.consume(r));
};


exports.js_js_rename = () => {
    (new wasm.JsRename()).bar();
    wasm.classes_foo();
};

exports.js_access_fields = () => {
    assert.ok((new wasm.AccessFieldFoo()).bar instanceof wasm.AccessFieldBar);
    assert.ok((new wasm.AccessField0())[0] instanceof wasm.AccessFieldBar);
};

exports.js_renamed_export = () => {
    const x = new wasm.JsRenamedExport();
    assert.ok(x.x === 3);
    x.foo();
    x.bar(x);
};

exports.js_conditional_bindings = () => {
    const x = new wasm.ConditionalBindings();
    x[wasm.__wbg_free]();
};

exports.js_assert_none = x => {
  assert.strictEqual(x, undefined);
};
exports.js_assert_some = x => {
  assert.ok(x instanceof wasm.OptionClass);
};
exports.js_return_none1 = () => null;
exports.js_return_none2 = () => undefined;
exports.js_return_some = x => x;

exports.js_test_option_classes = () => {
  assert.strictEqual(wasm.option_class_none(), undefined);
  wasm.option_class_assert_none(undefined);
  wasm.option_class_assert_none(null);
  const c = wasm.option_class_some();
  assert.ok(c instanceof wasm.OptionClass);
  wasm.option_class_assert_some(c);
};

exports.ExportedClass = class {
    constructor(...args) {
        this.super_args = args;
    }
}

exports.js_exported_class_inheritance = function() {
    assert.strictEqual(Object.getPrototypeOf(wasm.Child.prototype), wasm.Parent.prototype);
    assert.strictEqual(Object.getPrototypeOf(wasm.CustomImport.prototype), exports.ExportedClass.prototype);
    assert.strictEqual(Object.getPrototypeOf(wasm.CustomDate.prototype), Date.prototype);
}

exports.js_exported_super_constructors = function() {
    assert.strictEqual(new wasm.Child().get_value(), 123);
    assert.deepEqual(new wasm.CustomImport().super_args, ["abc", 123, 3.141, null, true]);
    assert.deepEqual(new wasm.CustomDate("hello"), new Date(2000,0));
}