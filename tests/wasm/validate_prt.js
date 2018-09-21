const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

const useMoved = () => {
    const apple = new wasm.Fruit('apple');
    apple.name();
    wasm.eat(apple);
    assert.throws(() => apple.name(),
      /Attempt to use a moved value|null pointer passed to rust/);
};

const moveMoved = () => {
    const pear = new wasm.Fruit('pear');
    pear.name();
    wasm.eat(pear);
    assert.throws(() => wasm.eat(pear),
      /Attempt to use a moved value|null pointer passed to rust/);
};

exports.js_works = () => {
    useMoved();
    moveMoved();
};
