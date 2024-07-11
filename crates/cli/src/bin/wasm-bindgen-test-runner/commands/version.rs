use anyhow::Result;

pub fn version() -> Result<()> {
    println!(
        "wasm-bindgen-test-runner {}",
        wasm_bindgen_shared::version()
    );
    return Ok(());
}
