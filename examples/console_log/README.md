# `console.log`

[View this example online](https://webassembly.studio/?f=ppd7u8k9i9)


This directory is an example of two ways to get access to the `console.log` function.
The first way uses the `#[wasm_bindgen]` macro to import the function and call it.
The second way uses the binding from the `web-sys` crate.

You can build the example with:

```
$ ./build.sh
```

(or running the commands on Windows manually)

and then opening up `index.html` in a web browser should show a dialog!

For more information about this example be sure to check out
[`hello_world`][hello] which also has more comments about caveats and such.

[hello]: https://github.com/rustwasm/wasm-bindgen/tree/master/examples/hello_world
