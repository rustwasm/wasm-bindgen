// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './hello_world';`
// will work here one day as well!
const js = import("./hello_world");

Promise.all([
  js,

  // Due to https://github.com/webpack/webpack/issues/6475, Webpack does not
  // support sync wasm imports larger than 4kB in Chrome. We use wasm2es6js to
  // hack around this and need to defer our call until the converted wasm
  // module is asynchronously loaded. Uncomment this line to enable.
  // This hack is not necessary in Firefox.
  // import("./hello_world_wasm.js").then(wasm => wasm.booted),
]).then(([js]) => {
  js.greet("World!");
});
