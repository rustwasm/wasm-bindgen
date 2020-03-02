use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

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
