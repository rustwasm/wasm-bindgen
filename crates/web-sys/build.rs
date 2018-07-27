extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate wasm_bindgen_webidl;
extern crate sourcefile;

use failure::{Fail, ResultExt};
use sourcefile::SourceFile;
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

    let mut source = SourceFile::default();
    for entry in entries {
        let entry = entry.context("getting webidls/enabled/*.webidl entry")?;
        if entry.path().extension() == Some(OsStr::new("webidl")) {
            println!("cargo:rerun-if-changed={}", entry.path().display());
            source = source.add_file(entry.path())
                .with_context(|_| format!("reading contents of file \"{}\"",
                                          entry.path().display()))?;
        }
    }

    let bindings = match wasm_bindgen_webidl::compile(&source.contents) {
        Ok(bindings) => bindings,
        Err(e) => match e.kind() {
            wasm_bindgen_webidl::ErrorKind::ParsingWebIDLSourcePos(pos) => {
                if let Some(pos) = source.resolve_offset(pos) {
                    let ctx = format!("compiling WebIDL into wasm-bindgen bindings in file \
                        \"{}\", line {} column {}", pos.filename, pos.line + 1, pos.col + 1);
                    return Err(e.context(ctx).into());
                } else {
                    return Err(e.context("compiling WebIDL into wasm-bindgen bindings").into());
                }
            }
            _ => {
                return Err(e.context("compiling WebIDL into wasm-bindgen bindings").into());
            }
        }
    };

    let out_dir = env::var("OUT_DIR").context("reading OUT_DIR environment variable")?;
    let out_file_path = path::Path::new(&out_dir).join("bindings.rs");
    let mut out_file = fs::File::create(&out_file_path)
        .context("creating output bindings file")?;
    out_file
        .write_all(bindings.as_bytes())
        .context("writing bindings to output file")?;

    // run rustfmt on the generated file - really handy for debugging
    //if ! process::Command::new("rustfmt").arg(&out_file_path).status()
    //    .context("running rustfmt")?.success() {
    //    return Err(format_err!("rustfmt failed to format {}", out_file_path.display()));
    //}

    Ok(())
}

