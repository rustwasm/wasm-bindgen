// synchronously, using the browser, import out shim JS scripts
importScripts('raytrace_parallel.js');

let booted = false;
let lastPtr = null;

// Wait for the main thread to send us the shared module/memory. Once we've got
// it, initialize it all with the `wasm_bindgen` global we imported via
// `importScripts`.
//
// After our first message all subsequent messages are an entry point to run,
// so we just do that.
self.onmessage = function(args) {
  self.onmessage = event => run(event.data);
  const [module, memory] = args.data;
  wasm_bindgen(module, memory)
    .then(() => {
      booted = true;
      if (lastPtr)
        run(lastPtr);
    })
    .catch(e => setTimeout(() => { throw e; })); // propagate to main `onerror`
};

function run(ptr) {
  if (!booted) {
    lastPtr = ptr;
    return;
  }
  lastPtr = null;
  wasm_bindgen.child_entry_point(ptr);
}
