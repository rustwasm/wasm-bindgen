#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate parity_wasm;
extern crate wasm_bindgen_cli_support;

use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;

use docopt::Docopt;

const USAGE: &'static str = "
Converts a wasm file to an ES6 JS module

Usage:
    wasm2es6js [options] <input>
    wasm2es6js -h | --help

Options:
    -h --help               Show this screen.
    -o --output FILE        File to place output in
    --typescript            Output a `*.d.ts` file next to the JS output
    --base64                Inline the wasm module using base64 encoding
    --fetch                 Load module via `fetch()` instead of default webpack implementation

Note that this is not intended to produce a production-ready output module
but rather is intended purely as a temporary \"hack\" until it's standard in
bundlers for working with wasm. Use this program with care!
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_output: Option<PathBuf>,
    flag_typescript: bool,
    flag_base64: bool,
    flag_fetch: bool,
    arg_input: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if !args.flag_base64 && !args.flag_fetch {
        panic!("unfortunately only works right now with base64 or fetch");
    }

    let mut wasm = Vec::new();
    File::open(&args.arg_input).expect("failed to open input")
        .read_to_end(&mut wasm).expect("failed to read input");

    let object = wasm_bindgen_cli_support::wasm2es6js::Config::new()
        .base64(args.flag_base64)
        .fetch(args.flag_fetch, &args.arg_input)
        .generate(&wasm)
        .expect("failed to parse wasm");

    if args.flag_typescript {
        if let Some(ref p) = args.flag_output {
            let dst = p.with_extension("d.ts");
            File::create(dst).expect("failed to create output")
                .write_all(object.typescript().as_bytes()).expect("failed to write output");
        }
    }

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
