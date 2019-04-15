global.GetNoInterfaceObject = class {
  static get() {
    return {
      number: 3,
      foo: () => {},
    }
  }
};
