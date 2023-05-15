// We only need `startup` here which is the main entry point
// In theory, we could also use all other functions/struct types from Rust which we have bound with
// `#[wasm_bindgen]`
const {startup} = wasm_bindgen;

async function run_wasm() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`
    // `wasm_bindgen` was imported in `index.html`
    await wasm_bindgen();

    console.log('index.js loaded');

    // Run main WASM entry point
    // This will create a worker from within our Rust code compiled to WASM
    startup();
}

run_wasm();
