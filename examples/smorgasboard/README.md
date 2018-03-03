# Smorgasboard of examples

This directory is an smattering of examples using the `#[wasm_bindgen]` macro.
Here we see passing strings back and for, exporting classes from Rust to JS,
importing classes from JS to Rust, etc.

You can build the example with:

```
$ ./build.sh
```

(or running the two commands on Windows manually)

and then opening up `index.html` in a web browser should show a dialog saying
"all passed" as well as some console output.
