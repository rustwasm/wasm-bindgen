# Polyfilling APIs

In JS new APIs often have polyfills via different names in various contexts. For
example the `AudioContext` API is known as `webkitAudioContext` in Safari at the
time of this writing. The `polyfill` attribute indicates these alternative
names.

For example to use `AudioContext` you might do:

```rust
#[wasm_bindgen]
extern {
    #[wasm_bindgen(polyfill = webkitAudioContext)]
    type AudioContext;

    // methods on `AudioContext` ...
}
```

Whenever `AudioContext` is used it'll use `AudioContext` if the global namespace
defines it or alternatively it'll fall back to `webkitAudioContext`.

Note that `polyfill` cannot be used with `module = "..."` or `js_namespace =
...`, so it's basically limited to web-platform APIs today.
