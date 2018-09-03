# WebAssembly to asm.js

This directory is an example of using [binaryen]'s `wasm2js` tool to convert
the wasm output of `wasm-bindgen` to a normal JS file that can be executed like
asm.js.

You can build the example locally with:

```
$ ./build.sh
```

When opened in a web browser this should print "Hello, World!" to the console.

Note that the `wasm2js` tool is still pretty early days so there's likely to be
a number of bugs to run into or work around. If any are encountered though
please feel free to report them upstream!

[binaryen]: https://github.com/WebAssembly/binaryen
