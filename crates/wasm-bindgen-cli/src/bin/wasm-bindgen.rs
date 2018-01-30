extern crate wasm_bindgen_cli_support;
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::path::PathBuf;

use docopt::Docopt;
use wasm_bindgen_cli_support::Bindgen;

const USAGE: &'static str = "
Generating JS bindings for a wasm file

Usage:
    wasm-bindgen [options] <input>
    wasm-bindgen -h | --help

Options:
    -h --help               Show this screen.
    --out-dir DIR           Output directory
    --nodejs                Generate output for node.js, not the browser
    --typescript            Output a TypeScript definition file
    --debug                 Include otherwise-extraneous debug checks in output
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_nodejs: bool,
    flag_typescript: bool,
    flag_out_dir: Option<PathBuf>,
    flag_debug: bool,
    arg_input: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut b = Bindgen::new();
    b.input_path(&args.arg_input)
     .nodejs(args.flag_nodejs)
     .debug(args.flag_debug)
     .typescript(args.flag_typescript);

    let out_dir = match args.flag_out_dir {
        Some(ref p) => p,
        None => panic!("the `--out-dir` argument is now required"),
    };

    b.generate(out_dir).expect("failed to generate bindings");
}
