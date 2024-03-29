# Support for Weak References

By default wasm-bindgen does use the [TC39 weak references
proposal](https://github.com/tc39/proposal-weakrefs) if support is detected.
At the time of this writing all major browsers do support it.

Without weak references your JS integration may be susceptible to memory leaks
in Rust, for example:

* You could forget to call `.free()` on a JS object, leaving the Rust memory
  allocated.
* Rust closures converted to JS values (the `Closure` type) may not be executed
  and cleaned up.
* Rust closures have `Closure::{into_js_value,forget}` methods which explicitly
  do not free the underlying memory.

These issues are all solved with the weak references proposal in JS.
`FinalizationRegistry` will ensure that all memory is cleaned up, regardless of
whether it's explicitly deallocated or not. Note that explicit deallocation
is always a possibility and supported, but if it's not called then memory will
still be automatically deallocated if `FinalizationRegistry` support is detected.
