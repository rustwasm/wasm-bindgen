# Importing non-browser JS

This directory is an example of using the `#[wasm_bindgen]` macro to import the
JS defined by you rather than the browser.

You can build the example with:

```
$ ./build.sh
```

(or running the commands on Windows manually)

and then opening up `index.html` in a web browser and see some messages in the
console.

For more information about this example be sure to check out
[`hello_world`][hello] which also has more comments about caveats and such.

[hello]: https://github.com/alexcrichton/wasm-bindgen/tree/master/examples/hello_world
