use anyhow::{Context, Result};
use sourcefile::SourceFile;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;
use wasm_bindgen_webidl::Feature;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "wasm-bindgen-webidl",
    about = "Converts WebIDL into wasm-bindgen compatible code."
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input_dir: PathBuf,

    #[structopt(parse(from_os_str))]
    output_dir: PathBuf,

    #[structopt(long)]
    no_features: bool,
}

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

fn rustfmt(path: &PathBuf, name: &str) -> Result<()> {
    // run rustfmt on the generated file - really handy for debugging
    let result = Command::new("rustfmt")
        .arg("--edition")
        .arg("2018")
        .arg(&path)
        .status()
        .context(format!("rustfmt on file {}", name))?;

    assert!(result.success(), "rustfmt on file {}", name);

    Ok(())
}

fn parse_webidl(
    opt: &Opt,
    enabled: SourceFile,
    unstable: SourceFile,
) -> Result<BTreeMap<String, Feature>> {
    let options = wasm_bindgen_webidl::Options {
        features: !opt.no_features,
    };

    match wasm_bindgen_webidl::compile(&enabled.contents, &unstable.contents, options) {
        Ok(features) => Ok(features),
        Err(e) => {
            if let Some(err) = e.downcast_ref::<wasm_bindgen_webidl::WebIDLParseError>() {
                if let Some(pos) = enabled.resolve_offset(err.0) {
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
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();

    let from = &opt.input_dir;
    let to = &opt.output_dir;

    let source = read_source_from_path(&from.join("enabled"))?;
    let unstable_source = read_source_from_path(&from.join("unstable"))?;

    let features = parse_webidl(&opt, source, unstable_source)?;

    if to.exists() {
        fs::remove_dir_all(&to).context("Removing features directory")?;
    }

    fs::create_dir_all(&to).context("Creating features directory")?;

    for (name, feature) in features.iter() {
        let out_file_path = to.join(format!("gen_{}.rs", name));

        fs::write(&out_file_path, &feature.code)?;

        rustfmt(&out_file_path, name)?;
    }

    let binding_file = features.keys().map(|name| {
        if opt.no_features {
            format!("mod gen_{name};\npub use gen_{name}::*;", name = name)
        } else {
            format!("#[cfg(feature = \"{name}\")] mod gen_{name};\n#[cfg(feature = \"{name}\")] pub use gen_{name}::*;", name = name)
        }
    }).collect::<Vec<_>>().join("\n\n");

    fs::write(
        to.join("mod.rs"),
        format!(
            "#![allow(non_snake_case, unused_imports)]\n\n{}",
            binding_file
        ),
    )?;

    rustfmt(&to.join("mod.rs"), "mod")?;

    if !opt.no_features {
        let features = features
            .iter()
            .map(|(name, feature)| {
                let features = feature
                    .required_features
                    .iter()
                    .map(|x| format!("\"{}\"", x))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} = [{}]", name, features)
            })
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&"features", features).context("writing features to current directory")?;
    }

    Ok(())
}
