const WASM: &[&str] = &[
    std::env!("CARGO_MANIFEST_DIR"),
    "/../../examples/wasm-in-wasm-imports/src/native_add.wasm",
];

mod common;

test_output_snapshot!(test_bundler_js, false, |b: &mut Bindgen| {
    b.bundler(true).unwrap();
});
test_output_empty!(test_bundler_start, true, |b: &mut Bindgen| {
    b.bundler(true).unwrap();
});
test_output_snapshot!(test_deno_js, false, |b: &mut Bindgen| {
    b.deno(true).unwrap();
});
test_output_empty!(test_deno_start, true, |b: &mut Bindgen| {
    b.deno(true).unwrap();
});
test_output_snapshot!(test_nodejs_js, false, |b: &mut Bindgen| {
    b.nodejs(true).unwrap();
});
test_output_empty!(test_nodejs_start, true, |b: &mut Bindgen| {
    b.nodejs(true).unwrap();
});
test_output_snapshot!(test_nodejs_module_js, false, |b: &mut Bindgen| {
    b.nodejs_module(true).unwrap();
});
test_output_snapshot!(test_nodejs_module_start, true, |b: &mut Bindgen| {
    b.nodejs_module(true).unwrap();
});
test_output_snapshot!(test_web_js, false, |b: &mut Bindgen| {
    b.web(true).unwrap();
});
test_output_empty!(test_web_start, true, |b: &mut Bindgen| {
    b.web(true).unwrap();
});
test_output_error!(test_nomodule_js, |b: &mut Bindgen| {
    // This test is expected to fail because the `no_modules` option cannot import
    b.no_modules(true).unwrap();
});
test_output_error!(test_nomodule_start, |b: &mut Bindgen| {
    // This test is expected to fail because the `no_modules` option cannot import
    b.no_modules(true).unwrap();
});
