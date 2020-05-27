# WebSockets Example

[View documentation for this example online][dox] or [View compiled example
online][compiled]

[compiled]: https://rustwasm.github.io/wasm-bindgen/exbuild/websockets/
[dox]: https://rustwasm.github.io/wasm-bindgen/examples/websockets.html

You can build the example locally with:

```
$ wasm-pack build --target web
```

Then serve the statics and navigate to `host:port`

```
# static server from https://crates.io/crates/https
http

# or use python
python2 -m SimpleHTTPServer
python3 -m http.server
```
