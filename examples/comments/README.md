# Comments

This directory is an example of how the `#[wasm_bindgen]` macro will
move your Rust doc comments to [JSDoc](http://usejsdoc.org/) comments

You can build the example locally with:

```
$ ./build.sh
```

(or running the two commands on Windows manually)

You should see the doc comments have been copied into the `comments.js` file.

If you wanted to run the project itself, simply run `npm run serve`