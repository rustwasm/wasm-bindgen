use failure::{Fail, ResultExt};
use sourcefile::SourceFile;
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{self, PathBuf};
use std::process::{self, Command};

fn main() {
    #[cfg(feature = "env_logger")]
    env_logger::init();

    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        for c in e.iter_causes() {
            eprintln!("  caused by {}", c);
        }
        process::exit(1);
    }
}

fn try_main() -> Result<(), failure::Error> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=webidls/enabled");

    let entries = fs::read_dir("webidls/enabled").context("reading webidls/enabled directory")?;
    let mut source = SourceFile::default();
    for entry in entries {
        let entry = entry.context("getting webidls/enabled/*.webidl entry")?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("webidl")) {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path.display());
        source = source
            .add_file(&path)
            .with_context(|_| format!("reading contents of file \"{}\"", path.display()))?;
    }

    // Read our manifest, learn all `[feature]` directives with "toml parsing".
    // Use all these names to match against environment variables set by Cargo
    // to figure out which features are activated to we can pass that down to
    // the webidl compiler.
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let manifest = fs::read_to_string(manifest_dir.join("Cargo.toml"))?;
    let features = manifest
        .lines()
        .skip_while(|f| !f.starts_with("[features]"));

    let enabled_features = env::vars()
        .map(|p| p.0)
        .filter(|p| p.starts_with("CARGO_FEATURE_"))
        .map(|mut p| {
            p.drain(0.."CARGO_FEATURE_".len());
            p
        })
        .collect::<HashSet<_>>();

    let mut allowed = Vec::new();
    for feature in features.filter(|f| !f.starts_with("#") && !f.starts_with("[")) {
        let mut parts = feature.split('=');
        let name = parts.next().unwrap().trim();
        if enabled_features.contains(&name.to_uppercase()) {
            allowed.push(name);
        }
    }

    // If we're printing all features don't filter anything
    println!("cargo:rerun-if-env-changed=__WASM_BINDGEN_DUMP_FEATURES");
    let allowed = if env::var("__WASM_BINDGEN_DUMP_FEATURES").is_ok() {
        None
    } else {
        Some(&allowed[..])
    };

    let bindings = match wasm_bindgen_webidl::compile(&source.contents, allowed) {
        Ok(bindings) => bindings,
        Err(e) => match e.kind() {
            wasm_bindgen_webidl::ErrorKind::ParsingWebIDLSourcePos(pos) => {
                if let Some(pos) = source.resolve_offset(pos) {
                    let ctx = format!(
                        "compiling WebIDL into wasm-bindgen bindings in file \
                         \"{}\", line {} column {}",
                        pos.filename,
                        pos.line + 1,
                        pos.col + 1
                    );
                    return Err(e.context(ctx).into());
                } else {
                    return Err(e
                        .context("compiling WebIDL into wasm-bindgen bindings")
                        .into());
                }
            }
            _ => {
                return Err(e
                    .context("compiling WebIDL into wasm-bindgen bindings")
                    .into());
            }
        },
    };

    let out_dir = env::var("OUT_DIR").context("reading OUT_DIR environment variable")?;
    let out_file_path = path::Path::new(&out_dir).join("bindings.rs");
    fs::write(&out_file_path, bindings).context("writing bindings to output file")?;
    println!("cargo:rustc-env=BINDINGS={}", out_file_path.display());

    // run rustfmt on the generated file - really handy for debugging
    //
    // This is opportunistic though so don't assert that it succeeds.
    println!("cargo:rerun-if-env-changed=WEBIDL_RUSTFMT_BINDINGS");
    if env::var("WEBIDL_RUSTFMT_BINDINGS").ok() != Some("0".to_string()) {
        drop(Command::new("rustfmt").arg(&out_file_path).status());
    }

    Ok(())
}
