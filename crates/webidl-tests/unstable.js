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
