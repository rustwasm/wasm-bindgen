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

global.NamedConstructor = class NamedConstructor {
  constructor() {
    this._value = 0;
  }

  get value(){
    return this._value;
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
