const wasmBooted = import('../wasm/production');

wasmBooted.then((wasmModule) => {
  const { greet } = wasmModule;
  greet(process.env.NODE_ENV!);
});
