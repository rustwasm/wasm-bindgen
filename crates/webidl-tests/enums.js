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
};
