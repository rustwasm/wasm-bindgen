# Examples of using `wasm-bindgen`, `js-sys`, and `web-sys`

This subsection contains examples of using the `wasm-bindgen`, `js-sys`, and
`web-sys` crates. Each example should have more information about what it's
doing.

The source code for all examples can also be [found online][code] to download an
run locally. Most examples are configured with Webpack/`wasm-pack` and can
be built with `npm run serve`. Other examples which don't use Webpack are
accompanied with a `build.sh` showing how to build it.

Note that most examples currently use Webpack to assemble the final output
artifact, but this is not required! You can use the bundler of choice,
`--no-modules`, or native browser ESM support as alternatives to Webpack.

[code]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples
