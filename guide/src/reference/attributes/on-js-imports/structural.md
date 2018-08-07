# `structural`

The `structural` flag can be added to `method` annotations, indicating that the
method being accessed (or property with getters/setters) should be accessed in a
structural, duck-type-y fashion. Rather than walking the constructor's prototype
chain once at load time and caching the property result, the prototype chain is
dynamically walked on every access.

```rust
#[wasm_bindgen]
extern {
    type Duck;

    #[wasm_bindgen(method, structural)]
    fn quack(this: &Duck);

    #[wasm_bindgen(method, getter, structural)]
    fn is_swimming(this: &Duck) -> bool;
}
```

The constructor for the type here, `Duck`, is not required to exist in
JavaScript (it's not referenced).  Instead `wasm-bindgen` will generate shims
that will access the passed in JavaScript value's `quack` method or its
`is_swimming` property.

```js
// Without `structural`, get the method directly off the prototype at load time:
const Duck_prototype_quack = Duck.prototype.quack;
function quack(duck) {
  Duck_prototype_quack.call(duck);
}

// With `structural`, walk the prototype chain on every access:
function quack(duck) {
  duck.quack();
}
```

## Why don't we always use the `structural` behavior?

In theory, it is faster since the prototype chain doesn't need to be traversed
every time the method or property is accessed, but today's optimizing JIT
compilers are really good about eliminating that cost. The real reason is to be
future compatible with the ["host bindings" proposal][host-bindings], which
requires that there be no JavaScript shim between the caller and the native host
function. In this scenario, the properties and methods *must* be resolved before
the wasm is instantiated.

[host-bindings]: https://github.com/WebAssembly/host-bindings/blob/master/proposals/host-bindings/Overview.md
