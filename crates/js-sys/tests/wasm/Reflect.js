exports.get_char_at = function() {
  return "foo".charAt;
};

exports.Rectangle = class {
  constructor(x, y){
    this.x = x,
    this.y = y
  }

  static eq(x, y) {
    return x === y;
  }
};

exports.Rectangle2 = class {
  constructor(x, y){
    this.x = x,
    this.y = y
  }

  static eq(x, y) {
    return x === y;
  }
};
