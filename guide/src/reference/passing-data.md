# Passing arbitrary data to JS

It's possible to pass arbirtrary data from Rust to JavaScript by serializing it
with [Serde](https://github.com/serde-rs/serde).

`wasm-bindgen` includes the `JsValue` type, which streamlines serializing and deserializing.

In order accomplish this, we must include the serde and serde_derive
crates in `Cargo.toml`, and configure `wasm-bindgen` to work with this feature:

Cargo.toml
```toml
[dependencies]
serde = "^1.0.59"
serde_derive = "^1.0.59"

[dependencies.wasm-bindgen]
version = "^0.2"
features = ["serde-serialize"]
```

In our top-level Rust file (eg `lib.rs` or `main.rs`), we enable the `Serialize`
macro:
```rust
#[macro_use]
extern crate serde_derive;
```

The data we pass at all nesting levels must be supported by serde, or be a `struct` or `enum` that
derives the Serialize trait. For example, let's say we'd like to pass this
struct to JS; doing so is not possible in bindgen directly due to the use
of public fields, `HashMap`s, arrays, and nested `Vec`s. Note that we do not
need to use the `#[wasm_bindgen]` macro.

```rust
#[derive(Serialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}
```

Here's a function that will pass an instance of this `struct` to JS:
```rust
#[wasm_bindgen]
pub fn pass_example() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.]
    };

    JsValue::from_serde(&example).unwrap()
}
```

When calling this function from JS, its output will automatically be deserialized.
In this example, `fied1` will be a JS `object` (Not a JS `Map`), `field2` will be a
2d JS `array`, and `field3` will be a 1d JS `array`. Example calling code:

```typescript
const rust = import("./from_rust");

rust.then(
    r => {
        console.log(r.pass_example())
    }
)
```
