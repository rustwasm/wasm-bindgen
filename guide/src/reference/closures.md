# Closures

The `#[wasm_bindgen]` attribute supports some Rust closures being passed to JS.
Examples of what you can do are:

```rust
#[wasm_bindgen]
extern {
    fn foo(a: &Fn()); // could also be `&mut FnMut()`
}
```

Here a function `foo` is imported from JS where the first argument is a *stack
closure*. You can call this function with a `&Fn()` argument and JS will receive
a JS function. When the `foo` function returns, however, the JS function will be
invalidated and any future usage of it will raise an exception.

Closures also support arguments and return values like exports do, for example:

```rust
#[wasm_bindgen]
extern {
    type Foo;

    fn bar(a: &Fn(u32, String) -> Foo);
}
```

Sometimes the stack behavior of these closures is not desired. For example you'd
like to schedule a closure to be run on the next turn of the event loop in JS
through `setTimeout`. For this you want the imported function to return but the
JS closure still needs to be valid!

To support this use case you can do:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn baz(a: &Closure<Fn()>);
}
```

The `Closure` type is defined in the `wasm_bindgen` crate and represents a "long
lived" closure. The JS closure passed to `baz` is still valid after `baz`
returns, and the validity of the JS closure is tied to the lifetime of the
`Closure` in Rust. Once `Closure` is dropped it will deallocate its internal
memory and invalidate the corresponding JS function.

Like stack closures a `Closure` also supports `FnMut`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn another(a: &Closure<FnMut() -> u32>);
}
```

At this time you cannot [pass a JS closure to Rust][cbjs], you can only pass a
Rust closure to JS in limited circumstances.

[cbjs]: https://github.com/rustwasm/wasm-bindgen/issues/103
