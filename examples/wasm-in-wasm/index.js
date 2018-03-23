// For more comments about what's going on here, check out the `hello_world`
// example
const rust = import("./wasm_in_wasm");
rust.then(m => m.run());
