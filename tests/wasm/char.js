const wasm = require('wasm-bindgen-test.js');

function assertEq(a, b) {
  console.log(a, '?=', b);
  if (a === b)
    return;
  throw new Error('not equal');
}

exports.char_works = function() {
  assertEq(wasm.char_single_char(), 'a');
  assertEq(wasm.char_wide_char(), '💩');
  assertEq(wasm.char_parrot('Ղ'), 'Ղ');
  assertEq(wasm.char_parrot('ҝ'), 'ҝ');
  assertEq(wasm.char_parrot('Δ'), 'Δ');
  assertEq(wasm.char_parrot('䉨'), '䉨');
  assertEq(wasm.char_round('a'), 'a');
  assertEq(wasm.char_round('㊻'), '㊻');
  wasm.char_short_test('a');
  wasm.char_wide_test('💩');
};

exports.js_parrot = function(a) { return a; };
