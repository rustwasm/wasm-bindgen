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

    let features = !opt.no_features;

    let generated_features = wasm_bindgen_webidl::generate(
        &opt.input_dir,
        &opt.output_dir,
        wasm_bindgen_webidl::Options { features },
    )?;

    if features {
        fs::write(&"features", generated_features)
            .context("writing features to current directory")?;
    }

    Ok(())
}
