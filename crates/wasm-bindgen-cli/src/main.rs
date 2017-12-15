extern crate wasm_bindgen;
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::path::PathBuf;

use docopt::Docopt;
use wasm_bindgen::Bindgen;

const USAGE: &'static str = "
Generating JS bindings for a wasm file

Usage:
    wasm-bindgen [options] <input>

Options:
    -h --help               Show this screen.
    --output-js FILE        Output JS file
    --output-wasm FILE      Output WASM file
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_output_js: Option<PathBuf>,
    flag_output_wasm: Option<PathBuf>,
    arg_input: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut b = Bindgen::new();
    b.input_path(&args.arg_input);
    let ret = b.generate().expect("failed to generate bindings");
    if let Some(ref js) = args.flag_output_js {
        ret.write_js_to(js).expect("failed to write JS output file");
    }
    if let Some(ref wasm) = args.flag_output_wasm {
        ret.write_wasm_to(wasm).expect("failed to write wasm output file");
    }
}
