# WebAssembly to asm.js

This directory is an example of using [binaryen]'s `wasm2asm` tool to convert
the wasm output of `wasm-bindgen` to a normal JS file that can be executed like
asm.js.

You can build the example locally with:

```
$ ./build.sh
```

When opened in a web browser this should print "Hello, World!" to the console.

This example uses the `wasm2es6js` tool to convert the wasm file to an ES module
that's implemented with asm.js instead of WebAssembly. The conversion to asm.js
is done by [binaryen]'s `wasm2asm` tool internally.

Note that the `wasm2asm` tool is still pretty early days so there's likely to be
a number of bugs to run into or work around. If any are encountered though
please feel free to report them upstream!

[binaryen]: https://github.com/WebAssembly/binaryen
