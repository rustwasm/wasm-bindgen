const wasm = require("wasm-bindgen-test.js");
const assert = require("assert");

async function gc() {
  if ("gc" in global) {
    global.gc();
    // Give the `FinalizationRegistry` callbacks a chance to run.
    await new Promise((resolve) => setTimeout(resolve, 0));
  } else {
    console.warn("test runner doesn't expose GC function");
  }
}

let dropCount = 0;
exports.drop_callback = () => dropCount += 1;

exports.owned_methods = async () => {
  dropCount = 0;
  for (let i = 0; i < 100; i++) {
    new wasm.OwnedValue(1).add(new wasm.OwnedValue(2)).n();
  }

  // The `OwnedValue`s should have been dropped as a result of `add` and `n`
  // taking ownership of `self`...
  assert.strictEqual(dropCount, 300);
  await gc();
  // ... and GCing shouldn't double-free them.
  assert.strictEqual(dropCount, 300);
};

// Make sure that objects created via. builders get GC'd properly.
exports.gc_builder = async () => {
  dropCount = 0;
  for (let i = 0; i < 100; i++) {
    wasm.OwnedValue.build(1);
  }

  await gc();
  assert.strictEqual(dropCount, 100);
};

// Make sure that objects created via. constructors get GC'd properly.
exports.gc_constructor = async () => {
  dropCount = 0;
  for (let i = 0; i < 100; i++) {
    new wasm.OwnedValue(1);
  }

  await gc();
  assert.strictEqual(dropCount, 100);
};
