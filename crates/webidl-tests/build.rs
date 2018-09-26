extern crate env_logger;
extern crate wasm_bindgen_webidl;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    env_logger::init();
    let idls = fs::read_dir(".")
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.extension().and_then(|s| s.to_str()) == Some("webidl"))
        .map(|f| (fs::read_to_string(&f).unwrap(), f));

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    for (i, (idl, path)) in idls.enumerate() {
        println!("processing {:?}", path);
        let mut generated_rust = wasm_bindgen_webidl::compile(&idl, None).unwrap();

        let out_file = out_dir.join(path.file_name().unwrap()).with_extension("rs");

        let js_file = path.with_extension("js").canonicalize().unwrap();
        generated_rust.push_str(&format!(
            r#"
            pub mod import_script {{
                use wasm_bindgen::prelude::*;
                use wasm_bindgen_test::*;

                #[wasm_bindgen(module = r"{}")]
                extern {{
                    fn not_actually_a_function{1}();
                }}

                #[wasm_bindgen_test]
                fn foo() {{
                    if ::std::env::var("NOT_GONNA_WORK").is_ok() {{
                        not_actually_a_function{1}();
                    }}
                }}
            }}
        "#,
            js_file.display(),
            i
        ));

        fs::write(&out_file, generated_rust).unwrap();

        // Attempt to run rustfmt, but don't worry if it fails or if it isn't
        // installed, this is just to help with debugging
        drop(Command::new("rustfmt").arg(&out_file).status());
    }
}
