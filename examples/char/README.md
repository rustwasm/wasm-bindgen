# Char

This directory is an example of how the `#[wasm_bindgen]` macro will convert the rust `char` type to a single code-point js `string`. 

You can build the example locally with:

```
$ ./build.sh
```

Opening your web browser should display a single counter with a random character for it's `key` and 0 for its `count`. You can click the `+` button to increase a counter's count. By clicking on the "add counter" button you should see a new counter added to the list with a different random character for it's `key`.

Under the hood javascript is choosing a random character from an Array of characters and passing that to the rust Counter struct's constructor so the character you are seeing on the page has made the full round trip from js to rust and back to js.