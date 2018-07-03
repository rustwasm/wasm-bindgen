# Customizing import behavior

The `#[wasm_bindgen]` macro supports a good amount of configuration for
controlling precisely how exports are exported and what they generate in JS.
This section is intended to hopefully be an exhaustive reference of the
possibilities!

* `readonly` - when attached to a `pub` struct field this indicates that it's
  readonly from JS and a setter will not be generated.

  ```rust
  #[wasm_bindgen]
  pub struct Foo {
      pub first: u32,
      #[wasm_bindgen(readonly)]
      pub second: u32,
  }
  ```

  Here the `first` field will be both readable and writable from JS, but the
  `second` field will be a `readonly` field in JS where the setter isn't
  implemented and attempting to set it will throw an exception.

* `constructor` - when attached to a Rust "constructor" it will make the
  generated JS bindings callable as `new Foo()`, for example:

  ```rust
  #[wasm_bindgen]
  pub struct Foo {
      contents: u32,
  }

  #[wasm_bindgen]
  impl Foo {
      #[wasm_bindgen(constructor)]
      pub fn new() -> Foo {
          Foo { contents: 0 }
      }

      pub fn get_contents(&self) -> u32 {
          self.contents
      }
  }
  ```

  Here this can be used in JS as:

  ```js
  import { Foo } from './my_module';

  const f = new Foo();
  console.log(f.get_contents());
  ```
