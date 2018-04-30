#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate parity_wasm;
extern crate wasm_bindgen_cli_support;
#[macro_use]
extern crate failure;

use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;
use std::process;

use docopt::Docopt;
use failure::{Error, ResultExt};

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
    --fetch PATH            Load module by passing the PATH argument to `fetch()`

Note that this is not intended to produce a production-ready output module
but rather is intended purely as a temporary \"hack\" until it's standard in
bundlers for working with wasm. Use this program with care!
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_output: Option<PathBuf>,
    flag_typescript: bool,
    flag_base64: bool,
    flag_fetch: Option<String>,
    arg_input: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let err = match rmain(&args) {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {}", err);
    for cause in err.causes().skip(1) {
        eprintln!("\tcaused by: {}", cause);
    }
    process::exit(1);
}

fn rmain(args: &Args) -> Result<(), Error> {
    if !args.flag_base64 && !args.flag_fetch.is_some() {
        bail!("unfortunately only works right now with base64 or fetch");
    }

    let mut wasm = Vec::new();
    File::open(&args.arg_input)
        .and_then(|mut f| f.read_to_end(&mut wasm))
        .with_context(|_| format!("failed to read `{}`", args.arg_input.display()))?;

    let object = wasm_bindgen_cli_support::wasm2es6js::Config::new()
        .base64(args.flag_base64)
        .fetch(args.flag_fetch.clone())
        .generate(&wasm)?;

    if args.flag_typescript {
        if let Some(ref p) = args.flag_output {
            let dst = p.with_extension("d.ts");
            let ts = object.typescript();
            File::create(&dst)
                .and_then(|mut f| f.write_all(ts.as_bytes()))
                .with_context(|_| {
                    format!("failed to write `{}`", dst.display())
                })?;
        }
    }

    let js = object.js()?;

    match args.flag_output {
        Some(ref p) => {
            File::create(p)
                .and_then(|mut f| f.write_all(js.as_bytes()))
                .with_context(|_| format!("failed to write `{}`", p.display()))?;
        }
        None => {
            println!("{}", js);
        }
    }

    Ok(())
}
