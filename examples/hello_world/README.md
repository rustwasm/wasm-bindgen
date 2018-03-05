# Hello, World!

This directory is an example of using the `#[wasm_bindgen]` macro to create an
entry point that's called from the browser and then displays a dialog.

You can build the example with:

```
$ ./build.sh
```

(or running the two commands on Windows manually)

and then opening up `index.html` in a web browser should show a dialog!

In Chrome, you'll need to delete hello_world_wasm.wasm after building (or
change hello_world.js to import hello_world_wasm.js instead) and uncomment the
relevant line in index.js to work around a webpack bug.
