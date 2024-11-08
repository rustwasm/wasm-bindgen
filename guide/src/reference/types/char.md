# `char`

| `T` parameter | `&T` parameter | `&mut T` parameter | `T` return value | `Option<T>` parameter | `Option<T>` return value | JavaScript representation |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Yes | No | No | Yes | Yes | Yes | A JavaScript string value |

Since JavaScript doesn't have a character type, `char` is represented as a JavaScript string with one Unicode code point.

> **Note**: [JavaScript strings uses UTF-16 encoding](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String#utf-16_characters_unicode_code_points_and_grapheme_clusters). This means that a single `char` may be represented by a string of length 1 or 2 in JavaScript, depending on the Unicode code point. See [`String.fromCodePoint`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/fromCodePoint) for more information.

When passed into Rust, the `char` value of a JavaScript string is determined using [`codePointAt(0)`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/codePointAt). If the JavaScript string is empty or starts with an unpaired surrogate, a runtime error will be thrown.

> **Note**: For more information about unpaired surrogates, see the [documentation for `str`](str.html).

## Example Rust Usage

```rust
{{#include ../../../../examples/guide-supported-types-examples/src/char.rs}}
```

## Example JavaScript Usage

```js
{{#include ../../../../examples/guide-supported-types-examples/char.js}}
```
