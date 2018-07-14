extern crate env_logger;
extern crate failure;
extern crate wasm_bindgen_webidl;

use failure::ResultExt;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path;
use std::process;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        for c in e.causes().skip(1) {
            eprintln!("  caused by {}", c);
        }
        process::exit(1);
    }
}

fn try_main() -> Result<(), failure::Error> {
    println!("cargo:rerun-if-changed=build.rs");
    env_logger::init();

    println!("cargo:rerun-if-changed=webidls/enabled");
    let entries = fs::read_dir("webidls/enabled").context("reading webidls/enabled directory")?;

    let mut contents = String::new();
    for entry in entries {
        let entry = entry.context("getting webidls/enabled/*.webidl entry")?;
        if entry.path().extension() == Some(OsStr::new("webidl")) {
            println!("cargo:rerun-if-changed={}", entry.path().display());

            let this_contents =
                fs::read_to_string(entry.path()).context("reading WebIDL file contents")?;
            contents.push_str(&this_contents);
        }
    }

    let bindings = wasm_bindgen_webidl::compile(&contents)
        .context("compiling WebIDL into wasm-bindgen bindings")?;

    let out_dir = env::var("OUT_DIR").context("reading OUT_DIR environment variable")?;
    let mut out_file = fs::File::create(path::Path::new(&out_dir).join("bindings.rs"))
        .context("creating output bindings file")?;
    out_file
        .write_all(bindings.as_bytes())
        .context("writing bindings to output file")?;

    Ok(())
}
