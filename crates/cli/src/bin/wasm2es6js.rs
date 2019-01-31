use docopt::Docopt;
use failure::{Error, ResultExt};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process;

// no need for jemalloc bloat in this binary (and we don't need speed)
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

const USAGE: &'static str = "
Converts a wasm file to an ES6 JS module

Usage:
    wasm2es6js [options] <input>
    wasm2es6js -h | --help

Options:
    -h --help               Show this screen.
    -o --output FILE        File to place output in
    --out-dir DIR           Directory to place ouptut in
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
    flag_out_dir: Option<PathBuf>,
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
    for cause in err.iter_causes() {
        eprintln!("\tcaused by: {}", cause);
    }
    process::exit(1);
}

fn rmain(args: &Args) -> Result<(), Error> {
    let wasm = fs::read(&args.arg_input)
        .with_context(|_| format!("failed to read `{}`", args.arg_input.display()))?;

    let object = wasm_bindgen_cli_support::wasm2es6js::Config::new()
        .base64(args.flag_base64)
        .fetch(args.flag_fetch.clone())
        .generate(&wasm)?;

    if args.flag_typescript {
        let ts = object.typescript()?;
        write(&args, "d.ts", ts.as_bytes(), false)?;
    }

    let (js, wasm) = object.js_and_wasm()?;

    write(args, "js", js.as_bytes(), false)?;
    if let Some(wasm) = wasm {
        write(args, "wasm", &wasm, false)?;
    }
    Ok(())
}

fn write(args: &Args, extension: &str, contents: &[u8], print_fallback: bool) -> Result<(), Error> {
    if let Some(p) = &args.flag_output {
        let dst = p.with_extension(extension);
        fs::write(&dst, contents)
            .with_context(|_| format!("failed to write `{}`", dst.display()))?;
    } else if let Some(p) = &args.flag_out_dir {
        let filename = args.arg_input.file_name().unwrap();
        let dst = p.join(filename).with_extension(extension);
        fs::write(&dst, contents)
            .with_context(|_| format!("failed to write `{}`", dst.display()))?;
    } else if print_fallback {
        println!("{}", String::from_utf8_lossy(contents))
    }

    Ok(())
}
