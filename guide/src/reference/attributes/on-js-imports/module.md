# `module = "blah"`

The `module` attributes configures the module from which items are imported. For
example,

```rust
#[wasm_bindgen(module = "wu/tang/clan")]
extern {
    type ThirtySixChambers;
}
```

generates JavaScript import glue like:

```js
import { ThirtySixChambers } from "wu/tang/clan";
```

If a `module` attribute is not present, then the global scope is used
instead. For example,

```rust
#[wasm_bindgen]
extern {
    fn illmatic() -> u32;
}
```

generates JavaScript import glue like:

```js
let illmatic = this.illmatic;
```
