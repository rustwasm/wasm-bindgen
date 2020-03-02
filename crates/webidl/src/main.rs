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

fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    let features =
        wasm_bindgen_webidl::generate(&opt.input_dir, &opt.output_dir, !opt.no_features)?;
    if !opt.no_features {
        fs::write(&"features", features).context("writing features to current directory")?;
    }
    Ok(())
}
