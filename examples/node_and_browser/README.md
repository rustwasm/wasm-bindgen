# Node and Browser


This directory is an example of using the `#[wasm_bindgen]` macro to create an
entry point that's callable from both the browser and Node.js. 

You can build the example locally with:

```
$ ./build.sh
```
This will build the project and spin up `webpack-dev-server` which makes it viewable at http://localhost:8080 
(or a higher port if 8080 is already bound).

To see this project used in Node.js you can exit the instance of `webpack-dev-server` and run

```
$ npm start
```

This will read in the `wasm-bindgen` (README.md)[../../README.md] file and convert it to HTML
you can find the results in `./out.html` once it is finished.

