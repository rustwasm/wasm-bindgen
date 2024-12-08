# `bool`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript boolean value |

> **Note**: Only [JavaScript `Boolean`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean) values (`true` or `false`) are supported when calling into Rust. If you want to pass truthy or falsy values to Rust, convert them to a boolean using `Boolean(value)` first.
>
> If you are using TypeScript, you don't have to worry about this, as TypeScript will emit a compiler error if you try to pass a non-`boolean` value.

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/bool.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/bool.js}}
```
