use super::ListArgs;
use super::RunArgs;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
// It would be better to use the default version, but it returns wasm-bindgen-cli instead of wasm-bindgen-test-runner
#[command(about = "Execute all wasm bindgen unit and integration tests and build examples of a local package", version = None, long_about = None)]
#[command(
    after_help = "Additional documentation: https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html"
)]
pub struct Args {
    /// The wasm file to test
    pub input: Option<PathBuf>,

    #[command(flatten)]
    pub run: RunArgs,

    #[command(flatten)]
    pub list: ListArgs,

    /// Print version
    #[arg(short = 'V', long)]
    pub version: bool,
}
