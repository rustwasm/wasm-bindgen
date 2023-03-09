const wasm = require('wasm');
const assert = require('assert');

const test = s => {
  wasm.set(s);
  assert.strictEqual(s, wasm.get());
};

test('');
test('x');
test('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa');
