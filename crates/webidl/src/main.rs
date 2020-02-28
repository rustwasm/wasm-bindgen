use anyhow::{Context, Result};
use sourcefile::SourceFile;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Read all WebIDL files in a directory into a single `SourceFile`
fn read_source_from_path(dir: &Path) -> Result<SourceFile> {
    let entries = fs::read_dir(dir).context("reading webidls directory")?;
    let mut source = SourceFile::default();
    for entry in entries {
        let entry = entry.context(format!("getting {}/*.webidl entry", dir.display()))?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("webidl")) {
            continue;
        }
        source = source
            .add_file(&path)
            .with_context(|| format!("reading contents of file \"{}\"", path.display()))?;
    }

    Ok(source)
}

fn parse_args() -> (String, String) {
    let mut args = std::env::args().into_iter();

    let _ = args.next().unwrap();
    let from = args.next().unwrap();
    let to = args.next().unwrap();

    (from, to)
}

fn main() -> Result<()> {
    env_logger::init();

    let (from, to) = parse_args();
    let from = Path::new(&from);
    let to = Path::new(&to);

    let source = read_source_from_path(&from.join("enabled"))?;
    let unstable_source = read_source_from_path(&from.join("unstable"))?;

    let features = match wasm_bindgen_webidl::compile(&source.contents, &unstable_source.contents) {
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


    if to.exists() {
        fs::remove_dir_all(&to).context("Removing features directory")?;
    }

    fs::create_dir_all(&to).context("Creating features directory")?;


    for (name, feature) in features.iter() {
        let out_file_path = to.join(format!("gen_{}.rs", name));

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

    fs::write(to.join("mod.rs"), format!("#![allow(non_snake_case)]\n\n{}", binding_file))?;


    let features = features.iter().map(|(name, feature)| {
        let features = feature.required_features.iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{} = [{}]", name, features)
    }).collect::<Vec<_>>().join("\n");

    fs::write(&"features", features).context("writing features to current directory")?;

    Ok(())
}
