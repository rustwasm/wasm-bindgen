# Hello, World!

This directory is an example of using the `#[wasm_bindgen]` macro to create an
entry point that's called from the browser and then displays a dialog.

You can build the example with:

```
$ ./build.sh
```

(or running the two commands on Windows manually)

and then opening up `index.html` in a web browser should show a dialog!

## Caveat for Chrome users

Note that unfortunately this example does not yet work in Chrome. Chrome has
different limits than than Firefox, for example, about instantiating wasm
modules. Currently the Webpack wasm integration uses `new WebAssembly.Instance`
which limits the input module to at most 4K, but frequently (especially in
development mode) wasm modules may be larger than 4K.

The usage of `new WebAssembly.Instance` is currently believed to [be a bug][bug]
in webpack which is likely to get fixed once [`instantiateStreaming`][bug2] is
used instead. Once this is [fixed in upstream Webpack][fix] then this example
with work in Chrome (like it does currently in Firefox).

In the meantime, however, there's a `chrome` directory in this folder which also
has a `build.sh` script that contains a workaround for this issue. If you're
using chrome it's recommended to `cd` into that folder and run that script.

The workaround here is a `wasm2es6js` tool, which is currently a bit of a hack.
The wasm-bindgen project assumes that wasm files are ES6 modules (as does
Webpack's current integration), so the `wasm2es6js` translates a wasm file to a
JS file by explicitly instantiating the wasm module rather than relying on the
bundler to do it. When doing this we can manually use
`WebAssemblyly.instantiate` which does not have similar limits in Chrome.

If all this seems unfortunate for now, don't worry because it should hopefully
be fixed soon! If you've got any questions about this though feel free to ask on
the issue tracker or in the `#rust-wasm` IRC channel.

[bug]: https://github.com/webpack/webpack/issues/6475
[bug2]: https://github.com/webpack/webpack/issues/6433
[fix]: https://github.com/webpack/webpack/pull/6709
