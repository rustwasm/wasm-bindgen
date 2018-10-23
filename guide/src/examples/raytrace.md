# Parallel Raytracing

[View full source code][code] or [view the compiled example online][online]

[online]: https://rustwasm.github.io/wasm-bindgen/exbuild/raytrace-parallel/
[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/raytrace-parallel

**This is an unstable and experimental example** of using threads with
WebAssembly and Rust, culminating in a parallel raytracer demo. Current
requirements for viewing this demo are:

* Firefox Nightly - other browsers haven't implemented the proposed WebAssembly
  features yet.
* `SharedArrayBuffer` is enabled in `about:config` in Firefox

This demo may also break over time as Firefox updates, but we'll try to keep it
working!

Locally to build this demo you'll need `xargo` and the `rust-src` rustup
component, and afterwards `./build.sh` like other examples should build the
example.

Again, to reiterate, this is all experimental and we're working through various
issues as we're working on this. If you're curious to see how this works it's
best to explore via the source code right now! More info will be available here
once WebAssembly threads are closer to stabilization.
