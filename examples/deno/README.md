# Using deno

You can build the example with

```sh
$ ./build.sh
```

and test it with

```sh
$ deno run --allow-read test.ts
```

The `--allow-read` flag is needed because the wasm file is read during runtime.
This will be fixed when https://github.com/denoland/deno/issues/5609 is resolved.
