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

Options:
    -h --help               Show this screen.
    --output-ts FILE        Output TypeScript file
    --output-wasm FILE      Output WASM file
    --nodejs                Generate output for node.js, not the browser
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_output_ts: Option<PathBuf>,
    flag_output_wasm: Option<PathBuf>,
    flag_nodejs: bool,
    arg_input: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut b = Bindgen::new();
    b.input_path(&args.arg_input);
    b.nodejs(args.flag_nodejs);
    let ret = b.generate().expect("failed to generate bindings");
    if let Some(ref ts) = args.flag_output_ts {
        ret.write_ts_to(ts).expect("failed to write TypeScript output file");
    } else {
        println!("{}", ret.generate_ts());
    }
    if let Some(ref wasm) = args.flag_output_wasm {
        ret.write_wasm_to(wasm).expect("failed to write wasm output file");
    }
}
