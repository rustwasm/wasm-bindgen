import { Foo, Bar, concat } from "./smorgasboard";

function assertEq(a, b) {
  if (a !== b)
    throw new Error(`${a} != ${b}`);
  console.log(`found ${a} === ${b}`);
}

assertEq(concat('a', 'b'), 'ab');

// Note the `new Foo()` syntax cannot be used, static function
// constructors must be used instead. Additionally objects allocated
// corresponding to Rust structs will need to be deallocated on the
// Rust side of things with an explicit call to `free`.
let foo = Foo.new();
assertEq(foo.add(10), 10);
foo.free();

// Pass objects to one another
let foo1 = Foo.new();
let bar = Bar.from_str("22", { opaque: 'object' });
foo1.add_other(bar);

// We also don't have to `free` the `bar` variable as this function is
// transferring ownership to `foo1`
bar.reset('34');
foo1.consume_other(bar);

assertEq(foo1.add(2), 22 + 34 + 2);
foo1.free();

alert('all passed!')
