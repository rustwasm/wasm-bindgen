# `getter` and `setter`

It is also possible to interact with `Rust` types either by using fields accessors. For example, the following:

```rust
#[wasm_bindgen]
extern "C" {
    fn check_modify_and_return_baz_in_js_fields(baz: Baz) -> Baz;
}

#[wasm_bindgen_test]
fn create_and_check_baz_in_rust() {
    let baz = check_modify_and_return_baz_in_js_fields(Baz { field: 123 });
    assert_eq!(baz.field.unwrap(), 456);
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Baz {
    field: i32,
}

#[wasm_bindgen]
impl Baz {
    #[wasm_bindgen(getter)]
    pub fn field(&self) -> i32 {
        self.field
    }

    #[wasm_bindgen(setter = field)]
    pub fn set_field(&mut self, field: i32) {
        self.field = field;
    }
}
```

Can be combined in `JavaScript` like in this snippet:

```js
check_modify_and_return_baz_in_js_fields = (baz) => {
    console.log(baz.field, 123);
    baz.field = 456;
    return baz;
};
```