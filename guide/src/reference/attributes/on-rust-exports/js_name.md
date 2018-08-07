# `js_name = Blah`

The `js_name` attribute can be used to export a different name in JS than what
something is named in Rust. It can be applied to both exported Rust functions
and types.

For example, this is often used to convert between Rust's snake-cased
identifiers into JavaScript's camel-cased identifiers:

```rust
#[wasm_bindgen(js_name = doTheThing)]
pub fn do_the_thing() -> u32 {
    42
}
```

This can be used in JavaScript as:

```js
import { doTheThing } from './my_module';

const x = doTheThing();
console.log(x);
```
