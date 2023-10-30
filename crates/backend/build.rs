fn main() {
    let enable_wasi_env_var = std::env::var("WASM_BINDGEN_ENABLE_WASI").unwrap_or_default();
    if enable_wasi_env_var == "1" {
        println!("cargo:rustc-cfg=enable_wasi");
    }
}
