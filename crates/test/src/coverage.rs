use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(wasm_bindgen_unstable_test_coverage)]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Vec<u8> {
    let mut coverage = Vec::new();
    // SAFETY: this function is not thread-safe, but our whole test runner is running single-threaded.
    unsafe {
        minicov::capture_coverage(&mut coverage).unwrap();
    }
    if coverage.is_empty() {
        console_error!(
            "Empty coverage data received. Make sure you compile the tests with
        RUSTFLAGS=\"-Cinstrument-coverage -Zno-profile-runtime --emit=llvm-ir\"",
        );
    }
    coverage
}

#[cfg(not(wasm_bindgen_unstable_test_coverage))]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Vec<u8> {
    Vec::new()
}
