// This `build.rs` makes `[package] links = ...` work in `Cargo.toml`.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let enable_wasi_env_var = std::env::var("WASM_BINDGEN_ENABLE_WASI").unwrap_or_default();
    if enable_wasi_env_var == "1" {
        println!("cargo:rustc-cfg=enable_wasi");
    }
}
