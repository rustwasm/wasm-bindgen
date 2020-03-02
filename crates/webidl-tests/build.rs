use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    wasm_bindgen_webidl::generate(
        "webidls".as_ref(),
        &out_dir,
        wasm_bindgen_webidl::Options { features: false },
    )
    .unwrap();
}
