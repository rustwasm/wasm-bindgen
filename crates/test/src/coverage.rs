use alloc::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "unstable-test-coverage")]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Option<Vec<u8>> {
    let mut coverage = Vec::new();

    if unstable_test_coverage::counters() > 0 {
        // SAFETY: this function is not thread-safe, but our whole test runner is running single-threaded.
        unsafe {
            unstable_test_coverage::capture_coverage(&mut coverage).unwrap();
        }

        Some(coverage)
    } else {
        None
    }
}

#[cfg(not(feature = "unstable-test-coverage"))]
#[wasm_bindgen]
pub fn __wbgtest_cov_dump() -> Option<Vec<u8>> {
    None
}
