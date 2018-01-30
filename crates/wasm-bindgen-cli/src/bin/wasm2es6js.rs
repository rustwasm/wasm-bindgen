#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate parity_wasm;
extern crate wasm_bindgen_cli_support;

use std::collections::HashSet;
use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;

use docopt::Docopt;
use parity_wasm::elements::*;

const USAGE: &'static str = "
Converts a wasm file to an ES6 JS module

Usage:
    wasm2es6js [options] <input>
    wasm2es6js -h | --help

Options:
    -h --help               Show this screen.
    -o --output FILE        File to place output in
    --base64                Inline the wasm module using base64 encoding
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_output: Option<PathBuf>,
    flag_base64: bool,
    arg_input: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if !args.flag_base64 {
        panic!("unfortunately only works right now with base64");
    }

    let mut wasm = Vec::new();
    File::open(&args.arg_input).expect("failed to open input")
        .read_to_end(&mut wasm).expect("failed to read input");

    let object = wasm_bindgen_cli_support::wasm2es6js::Config::new()
        .base64(args.flag_base64)
        .generate(&wasm)
        .expect("failed to parse wasm");
    let js = object.js();

    match args.flag_output {
        Some(ref p) => {
            File::create(p).expect("failed to create output")
                .write_all(js.as_bytes()).expect("failed to write output");
        }
        None => {
            println!("{}", js);
        }
    }
}
