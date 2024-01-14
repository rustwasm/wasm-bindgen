use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "coverage")]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Vec<u8> {
    let mut coverage = Vec::new();
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

/// Called when setting WASM_BINDGEN_TEST_COVERAGE but coverage feature is disabled.
/// Currently not being used because of issues in the interpreter regarding profiling
/// information which cause an error before we get here.
#[cfg(not(feature = "coverage"))]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Vec<u8> {
    console_error!(
        "Coverage was supposed to be dumped, but the \"coverage\" feature is disabled in wasm-bindgen-test",
    );
    Vec::new()
}
