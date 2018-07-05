import { Foo, Bar, concat } from "./smorgasboard";

function assertEq(a, b) {
  if (a !== b)
    throw new Error(`${a} != ${b}`);
  console.log(`found ${a} === ${b}`);
}

assertEq(concat('a', 'b'), 'ab');

// Note that to use `new Foo()` the constructor function must be annotated
// with `#[wasm_bindgen(constructor)]`, otherwise only `Foo.new()` can be used.
// Additionally objects allocated corresponding to Rust structs will need to
// be deallocated on the Rust side of things with an explicit call to `free`.
let foo = new Foo();
assertEq(foo.add(10), 10);
foo.free();

// Pass objects to one another
let foo1 = new Foo();
let bar = Bar.from_str("22", { opaque: 'object' });
foo1.add_other(bar);

// We also don't have to `free` the `bar` variable as this function is
// transferring ownership to `foo1`
bar.reset('34');
foo1.consume_other(bar);

assertEq(foo1.add(2), 22 + 34 + 2);
foo1.free();

alert('all passed!')
