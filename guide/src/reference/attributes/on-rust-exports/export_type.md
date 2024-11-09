# `export_type`

> **Note:** This attribute is only available in `wasmbindgen` version 0.2.96 and later.

By default, string enums do not generate any publicly exported runtime bindings and type definitions. This is done because libraries like `web-sys` can use string enums to represent specific JavaScript strings values in DOM APIs (example). If string enums were publicly exported by default, a few dozen string enum type definitions would be added to the public API of all crates that use `web-sys`.

However, this presents the problem that you cannot make a string enum public if you want to use it in your own public API. For example:

```rust
// your Rust code

#[wasm_bindgen]
pub enum Status {
    Success = "success",
    Failure = "failure",
}

#[wasm_bindgen]
pub fn get_status() -> Status {
    Status::Success
}
```

```ts
// the generated TypeScript bindings

type Status = "success" | "failure";
export function get_status(): Status;
```

As you can see, a type was generated for the `Status` string enum, but the type is not publicly exported.

While crafty users can work around this by defining their own alias (e.g. as `ReturnType<typeof get_status>`), the TypeScript API is less ergonomic because `Status` is not directly available.

The `export_type` attribute can be used to override this behavior and make the string enum publicly exported:

```rust
// your Rust code

#[wasm_bindgen(export_type)]
pub enum Status {
    Success = "success",
    Failure = "failure",
}

#[wasm_bindgen]
pub fn get_status() -> Status {
    Status::Success
}
```

```ts
// the generated TypeScript bindings

export type Status = "success" | "failure";
export function get_status(): Status;
```

## Interaction with `skip_typescript`

If you use the [`skip_typescript` attribute](./skip_typescript.md) on a string enum, the `export_type` attribute will be ignored. No type definition will be generated for the string enum.

```rust
// your Rust code

#[wasm_bindgen(skip_typescript, export_type)]
pub enum Status {
    Success = "success",
    Failure = "failure",
}

#[wasm_bindgen]
pub fn get_status() -> Status {
    Status::Success
}
```

```ts
// the generated TypeScript bindings

export function get_status(): Status;
```

Note that this type definition has a type error, because `Status` is not defined. Using `skip_typescript` almost always requires you to define your own TypeScript types with [`typescript_custom_section`](./typescript_custom_section.md).
