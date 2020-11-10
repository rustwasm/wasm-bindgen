const strictEqual = require('assert').strictEqual;

exports.noop = function () {};

global.TestArrays = class {
  strings(x) {
    strictEqual(x, 'y');
    return 'x';
  }
  byteStrings(x) {
    strictEqual(x, 'yz');
    return 'xx';
  }
  usvStrings(x) {
    strictEqual(x, 'abc');
    return 'efg';
  }
  f32(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Float32Array([3, 4, 5]);
  }
  f64(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Float64Array([3, 4, 5]);
  }
  i8(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Int8Array([3, 4, 5]);
  }
  i16(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Int16Array([3, 4, 5]);
  }
  i32(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Int32Array([3, 4, 5]);
  }
  u8(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint8Array([3, 4, 5]);
  }
  u8Clamped(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint8ClampedArray([3, 4, 5]);
  }
  u16(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint16Array([3, 4, 5]);
  }
  u32(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint32Array([3, 4, 5]);
  }
  get octetArray() {
    return new Uint8Array([3, 4, 5]);
  }

  get octetSequence() {
    return new Uint8Array([3, 4, 5]);
  }
};

global.ArrayBufferTest = class {
  getBuffer() {
    return new ArrayBuffer(3);
  }
  setBuffer(x) {
    // ...
  }
  getDataView() {
    // taken from MDN
    const buffer = new ArrayBuffer(16);
    const view1 = new DataView(buffer);
    const view2 = new DataView(buffer, 12, 4);
    view1.setInt8(12, 42);

    return view2;
  }
};

global.TakeCallbackInterface = class {
  a() {}
  b() {}
};

global.assert_dict_c = function(c) {
  strictEqual(c.a, 1);
  strictEqual(c.b, 2);
  strictEqual(c.c, 3);
  strictEqual(c.d, 4);
  strictEqual(c.e, 5);
  strictEqual(c.f, 6);
  strictEqual(c.g, 7);
  strictEqual(c.h, 8);
};

global.mk_dict_a = function() {
  return {};
};

global.assert_dict_required = function(c) {
  strictEqual(c.a, 3);
  strictEqual(c.b, "a");
  strictEqual(c.c, 4);
};

global.assert_camel_case = function(dict) {
  strictEqual(dict.wierd_fieldName, 1);
}

global.Shape = class Shape {
  constructor(kind) {
    this.kind = kind;
  }

  static triangle() {
    return new Shape('triangle');
  }

  isSquare() {
    return this.kind === 'square';
  }

  isCircle() {
    return this.kind === 'circle';
  }

  getShape() {
    return this.kind;
  }

  get shapeTypeNone() {
    return null;
  }

  get shapeTypeSome() {
    return this.kind;
  }
};

const map = {
  global_no_args: () => 3,
  global_with_args: (a, b) => a + b,
  global_attribute: 'x',
};

global.get_global = () => map;

global.math_test = {
  pow(base, exp) {
    return Math.pow(base, exp);
  },

  add_one(val) {
    return val + 1;
  },
};

global.GetNoInterfaceObject = class {
  static get() {
    return {
      number: 3,
      foo: () => {},
    }
  }
};

global.Method = class Method {
  constructor(value) {
    this.value = value;
  }
  myCmp(other) {
    return this.value === other.value;
  }
};

global.Property = class Property {
  constructor(value) {
    this._value = value;
  }

  get value() {
    return this._value;
  }

  set value(value) {
    this._value = value;
  }
};

global.NamedConstructorParent = class NamedConstructor {
  constructor() {
    this._value = 0;
  }

  get value(){
    return this._value;
  }
};

global.NamedConstructor = class NamedConstructor extends NamedConstructorParent {
  constructor(_value) {
    super();
    this._value = _value;
  }
};

global.NamedConstructorBar = class NamedConstructorBar extends NamedConstructor {
  constructor(_value) {
    super();
    this._value = _value;
  }
};

global.StaticMethod = class StaticMethod {
  static swap(value) {
    const res = StaticMethod.value;
    StaticMethod.value = value;
    return res;
  }
};

StaticMethod.value = 0;

global.StaticProperty = class StaticProperty {
  static get value(){
    return StaticProperty._value;
  }

  static set value(value) {
    StaticProperty._value = value;
  }
};

StaticProperty._value = 0;

global.UndefinedMethod = class UndefinedMethod {
  constructor() {}
  ok_method() {
    return true;
  }
};

global.NullableMethod = class NullableMethod {
  constructor() {}
  opt(a) {
    if (a == undefined) {
      return undefined;
    } else {
      return a + 1;
    }
  }
};

global.Unforgeable = class Unforgeable {
  constructor() {
    this.uno = 1;
  }
  get dos() {
    return 2;
  }
};

global.GlobalMethod = class {
  m() { return 123; }
};

global.get_global_method = () => new GlobalMethod();

global.Indexing = function () {
  return new Proxy({}, {
    get(obj, prop) {
      return obj.hasOwnProperty(prop) ? obj[prop] : -1;
    },
    set(obj, prop, value) {
      obj[prop] = value;
    },
    deleteProperty(obj, prop) {
      delete obj[prop];
    },
  });
};

global.OptionalAndUnionArguments = class OptionalAndUnionArguments {
  constructor() {}
  m(a, b = true, c = 123, d = 456) {
    return [typeof a, a, typeof b, b, typeof c, c, typeof d, d].join(', ');
  }
};

global.Variadic = class Variadic {
  constructor() {}
  sum(...values) {
    return values.reduce((a, b) => a + b, 0);
  }
};

global.PartialInterface = class PartialInterface {
  get un() {
    return 1;
  }
  deux() {
    return 2;
  }
  get trois() {
    return 3;
  }
  quatre() {
    return 4;
  }
};

global.MixinFoo = class MixinFoo {
  constructor(bar) {
    this._bar = bar | MixinFoo.defaultBar;
  }
  static get defaultBar() {
    return MixinFoo._defaultBar;
  }
  static set defaultBar(defaultBar) {
    MixinFoo._defaultBar = defaultBar;
  }
  get bar() {
    return this._bar;
  }
  addToBar(other) {
    this._bar += other;
  }
};

global.Overloads = class {
  foo() {}
};

global.InvokeCallback = class {
  invoke(f) { f(); }
  callAdd(f) {
    return f(1, 2);
  }
  callRepeat(f) {
    return f('ab', 4);
  }
};

global.Thang = class Thang {
  constructor(value) {
    if (value % 2 == 0) {
      throw new Error("only odd allowed");
    }
    this.value = value;
  }

  get ok_attr() { return this.value; }
  set ok_attr(x) { }

  get err_attr() { throw new Error("bad"); }
  set err_attr(x) { throw new Error("bad"); }

  ok_method() { return this.value + 1; }
  err_method() { throw new Error("bad"); }

  static ok_static_method() { return 1; }
  static err_static_method() { throw new Error("bad"); }

  static get ok_static_attr() { return 1; }
  static set ok_static_attr(x) { }

  static get err_static_attr() { throw new Error("bad"); }
  static set err_static_attr(x) { throw new Error("bad"); }
};

global.GetUnstableInterface = class {
  static get() {
    return {
      enum_value(dict) {
        if (!dict) {
          return 0;
        }

        return dict.unstableEnum === "a" ? 1 : 2;
      }
    }
  }
}

global.TestPromises = class {
  stringPromise() {
    return new Promise(r => r("abc"));
  }
};
