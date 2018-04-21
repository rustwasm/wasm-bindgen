# Adding Numbers

[View this example online](https://webassembly.studio/?f=612vwsrmwft)

This directory is an example of using the `#[wasm_bindgen]` macro to simply add
two numbers. The neat part about this is that it's an example of how to generate
the smallest wasm-bindgen binary.

You can build the example with:

```
$ ./build.sh
```

(or running the commands on Windows manually)

Currently this generates a 651 byte wasm binary:

```
$ ls -alh add_bg.wasm
-rw-rw-r-- 1 alex alex 651 Apr 20 22:16 add_bg.wasm
```

If you run [wasm-opt], a C++ tool for optimize WebAssembly, you can make it even
smaller too!

```
$ wasm-opt -Os add_bg.wasm -o add.wasm
$ ls -alh add.wasm
-rw-rw-r-- 1 alex alex 100 Apr 20 22:19 add.wasm
```

And sure enough, using the [wasm2wat] tool it's quite small!

```
$ wasm2wat add.wasm
(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    get_local 1
    get_local 0
    i32.add)
  (memory (;0;) 2)
  (export "memory" (memory 0))
  (export "add" (func 0))
(data (i32.const 1545) "invalid malloc request"))
```

Note that it's important to point out that the size reductions here are because
the wasm is compiled in release mode by the build script and this crate's
workspace has the following configuration

```toml
[profile.release]
lto = true
opt-level = 's'
panic = 'abort'
```

[wasm2wat]: https://github.com/webassembly/wabt
[wasm-opt]: https://github.com/webassembly/binaryen
