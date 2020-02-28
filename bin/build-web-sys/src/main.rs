use anyhow::{Context, Result};
use sourcefile::SourceFile;
use std::ffi::OsStr;
use std::fs;
use std::path;
use std::process::Command;

fn unwrap_not_found(err: std::io::Result<()>) -> std::io::Result<()> {
    match err {
        Ok(()) => Ok(()),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Ok(()),
            _ => Err(err),
        },
    }
}

fn main() -> Result<()> {
    #[cfg(feature = "env_logger")]
    env_logger::init();

    let web_sys_dir = path::Path::new("../../crates/web-sys");

    let entries = fs::read_dir(web_sys_dir.join("webidls/enabled")).context("reading webidls/enabled directory")?;

    let mut source = SourceFile::default();

    for entry in entries {
        let entry = entry.context("getting webidls/enabled/*.webidl entry")?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("webidl")) {
            continue;
        }
        source = source
            .add_file(&path)
            .with_context(|| format!("reading contents of file \"{}\"", path.display()))?;
    }

    let features = match wasm_bindgen_webidl::compile(&source.contents) {
        Ok(features) => features,
        Err(e) => {
            if let Some(err) = e.downcast_ref::<wasm_bindgen_webidl::WebIDLParseError>() {
                if let Some(pos) = source.resolve_offset(err.0) {
                    let ctx = format!(
                        "compiling WebIDL into wasm-bindgen bindings in file \
                         \"{}\", line {} column {}",
                        pos.filename,
                        pos.line + 1,
                        pos.col + 1
                    );
                    return Err(e.context(ctx));
                } else {
                    return Err(e.context("compiling WebIDL into wasm-bindgen bindings"));
                }
            }
            return Err(e.context("compiling WebIDL into wasm-bindgen bindings"));
        }
    };


    let binding_dir = web_sys_dir.join("src/features");

    unwrap_not_found(fs::remove_dir_all(&binding_dir)).context("Removing features directory")?;
    fs::create_dir_all(&binding_dir).context("Creating features directory")?;


    for (name, feature) in features.iter() {
        let out_file_path = binding_dir.join(format!("gen_{}.rs", name));

        fs::write(&out_file_path, &feature.code)?;

        // run rustfmt on the generated file - really handy for debugging
        let result = Command::new("rustfmt")
            .arg("--edition")
            .arg("2018")
            .arg(&out_file_path)
            .status()
            .context(format!("rustfmt on file gen_{}.rs", name))?;

        assert!(result.success(), "rustfmt on file gen_{}.rs", name);
    }


    let binding_file = features.keys().map(|name| {
        format!("#[cfg(feature = \"{name}\")] mod gen_{name};\n#[cfg(feature = \"{name}\")] pub use gen_{name}::*;", name = name)
    }).collect::<Vec<_>>().join("\n\n");

    fs::write(binding_dir.join("mod.rs"), format!("#![allow(non_snake_case)]\n\n{}", binding_file))?;


    let features = features.iter().map(|(name, feature)| {
        let features = feature.required_features.iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{} = [{}]", name, features)
    }).collect::<Vec<_>>().join("\n");

    fs::write(&web_sys_dir.join("features"), features).context("writing features to output file")?;

    Ok(())
}
