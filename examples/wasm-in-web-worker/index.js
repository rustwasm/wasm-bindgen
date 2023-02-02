async function run_wasm() {
    console.log('index.js loaded');

    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`
    // `wasm_bindgen` was imported in `index.html`
    await wasm_bindgen('./pkg/wasm_in_web_worker_bg.wasm');
}

run_wasm();
