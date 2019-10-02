# `constructor`

Every exported type **must** have exactly one associated method that is annotated with `#[wasm_bindgen(constructor)]`.  The generated JavaScript shim will then bind its constructor to that method, such that objects can be instantiated from JavaScript with `new Foo()` etc.

Every constructor method **must** call the `instantiate! {}` macro exactly once.  The body of the macro invocation is:

1. a call to `super();` (but only if the struct being constructed specified a [`prototype`](constructor.html)); any number of arguments can be provided, each of which can be of any type that is exportable to JavaScript; the terminating semicolon is required.

2. a literal instantiation of the type being constructed.

The macro evaluates to a `WasmType<T>` of the constructed object; and **such value will automatically be returned by the associated method irrespective of the function's final expression or any explicit `return` statement** (the values of which are discarded).

For example, consider this exported Rust type and `constructor` annotation:

```rust
#[wasm_bindgen]
pub struct Foo {
    contents: u32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<Foo> {
        instantiate! { Foo { contents: 0 } }
    }

    pub fn get_contents(&self) -> u32 {
        self.contents
    }
}
```

This can be used in JavaScript as:

```js
import { Foo } from './my_module';

const f = new Foo();
console.log(f.get_contents());
```