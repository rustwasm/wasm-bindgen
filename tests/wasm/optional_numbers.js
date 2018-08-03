const assert = require('assert');
const wasm = require('wasm-bindgen-test.js');

exports.test_works = function() {
  assert.strictEqual(wasm.i32_identity(wasm.i32_none()), undefined);
  assert.strictEqual(wasm.i32_identity(wasm.i32_zero()), 0);
  assert.strictEqual(wasm.i32_identity(wasm.i32_one()), 1);
  assert.strictEqual(wasm.i32_identity(wasm.i32_neg_one()), -1);
  assert.strictEqual(wasm.i32_identity(wasm.i32_max()), 2147483647);
  assert.strictEqual(wasm.i32_identity(wasm.i32_min()), -2147483648);

  assert.strictEqual(wasm.u32_identity(wasm.u32_none()), undefined);
  assert.strictEqual(wasm.u32_identity(wasm.u32_zero()), 0);
  assert.strictEqual(wasm.u32_identity(wasm.u32_one()), 1);
  assert.strictEqual(wasm.u32_identity(wasm.u32_max()), 4294967295);
  assert.strictEqual(wasm.u32_identity(wasm.u32_min()), 0);

  assert.strictEqual(wasm.isize_identity(wasm.isize_none()), undefined);
  assert.strictEqual(wasm.isize_identity(wasm.isize_zero()), 0);
  assert.strictEqual(wasm.isize_identity(wasm.isize_one()), 1);
  assert.strictEqual(wasm.isize_identity(wasm.isize_neg_one()), -1);
  assert.strictEqual(wasm.isize_identity(wasm.isize_max()), 2147483647);
  assert.strictEqual(wasm.isize_identity(wasm.isize_min()), -2147483648);

  assert.strictEqual(wasm.usize_identity(wasm.usize_none()), undefined);
  assert.strictEqual(wasm.usize_identity(wasm.usize_zero()), 0);
  assert.strictEqual(wasm.usize_identity(wasm.usize_one()), 1);
  assert.strictEqual(wasm.usize_identity(wasm.usize_max()), 4294967295);
  assert.strictEqual(wasm.usize_identity(wasm.usize_min()), 0);

  assert.strictEqual(wasm.f32_identity(wasm.f32_none()), undefined);
  assert.strictEqual(wasm.f32_identity(wasm.f32_zero()), 0);
  assert.strictEqual(wasm.f32_identity(wasm.f32_one()), 1);
  assert.strictEqual(wasm.f32_identity(wasm.f32_neg_one()), -1);

  assert.strictEqual(wasm.f64_identity(wasm.f64_none()), undefined);
  assert.strictEqual(wasm.f64_identity(wasm.f64_zero()), 0);
  assert.strictEqual(wasm.f64_identity(wasm.f64_one()), 1);
  assert.strictEqual(wasm.f64_identity(wasm.f64_neg_one()), -1);

  assert.strictEqual(wasm.i8_identity(wasm.i8_none()), undefined);
  assert.strictEqual(wasm.i8_identity(wasm.i8_zero()), 0);
  assert.strictEqual(wasm.i8_identity(wasm.i8_one()), 1);
  assert.strictEqual(wasm.i8_identity(wasm.i8_neg_one()), -1);
  assert.strictEqual(wasm.i8_identity(wasm.i8_max()), 127);
  assert.strictEqual(wasm.i8_identity(wasm.i8_min()), -128);

  assert.strictEqual(wasm.u8_identity(wasm.u8_none()), undefined);
  assert.strictEqual(wasm.u8_identity(wasm.u8_zero()), 0);
  assert.strictEqual(wasm.u8_identity(wasm.u8_one()), 1);
  assert.strictEqual(wasm.u8_identity(wasm.u8_max()), 255);
  assert.strictEqual(wasm.u8_identity(wasm.u8_min()), 0);

  assert.strictEqual(wasm.i16_identity(wasm.i16_none()), undefined);
  assert.strictEqual(wasm.i16_identity(wasm.i16_zero()), 0);
  assert.strictEqual(wasm.i16_identity(wasm.i16_one()), 1);
  assert.strictEqual(wasm.i16_identity(wasm.i16_neg_one()), -1);
  assert.strictEqual(wasm.i16_identity(wasm.i16_max()), 32767);
  assert.strictEqual(wasm.i16_identity(wasm.i16_min()), -32768);

  assert.strictEqual(wasm.u16_identity(wasm.u16_none()), undefined);
  assert.strictEqual(wasm.u16_identity(wasm.u16_zero()), 0);
  assert.strictEqual(wasm.u16_identity(wasm.u16_one()), 1);
  assert.strictEqual(wasm.u16_identity(wasm.u16_max()), 65535);
  assert.strictEqual(wasm.u16_identity(wasm.u16_min()), 0);

  assert.strictEqual(wasm.i64_identity(wasm.i64_none()), undefined);
  assert.strictEqual(wasm.i64_identity(wasm.i64_zero()), BigInt('0'));
  assert.strictEqual(wasm.i64_identity(wasm.i64_one()), BigInt('1'));
  assert.strictEqual(wasm.i64_identity(wasm.i64_neg_one()), BigInt('-1'));
  assert.strictEqual(wasm.i64_identity(wasm.i64_max()), BigInt('9223372036854775807'));
  assert.strictEqual(wasm.i64_identity(wasm.i64_min()), BigInt('-9223372036854775808'));

  assert.strictEqual(wasm.u64_identity(wasm.u64_none()), undefined);
  assert.strictEqual(wasm.u64_identity(wasm.u64_zero()), BigInt('0'));
  assert.strictEqual(wasm.u64_identity(wasm.u64_one()), BigInt('1'));
  assert.strictEqual(wasm.u64_identity(wasm.u64_max()), BigInt('18446744073709551615'));
  assert.strictEqual(wasm.u64_identity(wasm.u64_min()), BigInt('0'));
};

exports.i32_js_identity = function(a) { return a; };
exports.u32_js_identity = function(a) { return a; };
exports.isize_js_identity = function(a) { return a; };
exports.usize_js_identity = function(a) { return a; };
exports.f32_js_identity = function(a) { return a; };
exports.f64_js_identity = function(a) { return a; };
exports.i8_js_identity = function(a) { return a; };
exports.u8_js_identity = function(a) { return a; };
exports.i16_js_identity = function(a) { return a; };
exports.u16_js_identity = function(a) { return a; };
exports.i64_js_identity = function(a) { return a; };
exports.u64_js_identity = function(a) { return a; };
