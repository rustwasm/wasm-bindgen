const strictEqual = require('assert').strictEqual;

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
};
