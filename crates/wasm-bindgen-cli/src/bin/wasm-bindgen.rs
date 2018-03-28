extern crate wasm_bindgen_cli_support;
#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate wasm_bindgen_shared;

use std::path::PathBuf;

use docopt::Docopt;
use wasm_bindgen_cli_support::Bindgen;

const USAGE: &'static str = "
Generating JS bindings for a wasm file

Usage:
    wasm-bindgen [options] <input>
    wasm-bindgen -h | --help
    wasm-bindgen -V | --version

Options:
    -h --help                Show this screen.
    --out-dir DIR            Output directory
    --nodejs                 Generate output that only works in node.js
    --browser                Generate output that only works in a browser
    --typescript             Output a TypeScript definition file
    --debug                  Include otherwise-extraneous debug checks in output
    -V --version             Print the version number of wasm-bindgen
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_nodejs: bool,
    flag_browser: bool,
    flag_typescript: bool,
    flag_out_dir: Option<PathBuf>,
    flag_debug: bool,
    flag_version: bool,
    arg_input: Option<PathBuf>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("wasm-bindgen {}", wasm_bindgen_shared::version());
        return
    }

    let input = match args.arg_input {
        Some(s) => s,
        None => panic!("input file expected"),
    };

    let mut b = Bindgen::new();
    b.input_path(&input)
     .nodejs(args.flag_nodejs)
     .browser(args.browser)
     .debug(args.flag_debug)
     .typescript(args.flag_typescript);

    let out_dir = match args.flag_out_dir {
        Some(ref p) => p,
        None => panic!("the `--out-dir` argument is now required"),
    };

    b.generate(out_dir).expect("failed to generate bindings");
}
