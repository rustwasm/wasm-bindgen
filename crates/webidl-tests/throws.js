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
