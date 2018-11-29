extern crate wasm_bindgen_cli_support;
#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate wasm_bindgen_shared;
#[macro_use]
extern crate failure;
extern crate env_logger;

use std::path::PathBuf;
use std::process;

use docopt::Docopt;
use failure::Error;
use wasm_bindgen_cli_support::Bindgen;

// no need for jemalloc bloat in this binary (and we don't need speed)
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

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
    --no-modules             Generate output that only works in a browser (without modules)
    --no-modules-global VAR  Name of the global variable to initialize
    --typescript             Output a TypeScript definition file (on by default)
    --no-typescript          Don't emit a *.d.ts file
    --debug                  Include otherwise-extraneous debug checks in output
    --no-demangle            Don't demangle Rust symbol names
    --keep-debug             Keep debug sections in wasm files
    --remove-name-section    Remove the debugging `name` section of the file
    -V --version             Print the version number of wasm-bindgen
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_nodejs: bool,
    flag_browser: bool,
    flag_no_modules: bool,
    flag_typescript: bool,
    flag_no_typescript: bool,
    flag_out_dir: Option<PathBuf>,
    flag_debug: bool,
    flag_version: bool,
    flag_no_demangle: bool,
    flag_no_modules_global: Option<String>,
    flag_remove_name_section: bool,
    flag_keep_debug: bool,
    arg_input: Option<PathBuf>,
}

fn main() {
    env_logger::init();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("wasm-bindgen {}", wasm_bindgen_shared::version());
        return;
    }
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
    let input = match args.arg_input {
        Some(ref s) => s,
        None => bail!("input file expected"),
    };

    let typescript = args.flag_typescript || !args.flag_no_typescript;

    let mut b = Bindgen::new();
    b.input_path(input)
        .nodejs(args.flag_nodejs)
        .browser(args.flag_browser)
        .no_modules(args.flag_no_modules)
        .debug(args.flag_debug)
        .demangle(!args.flag_no_demangle)
        .keep_debug(args.flag_keep_debug)
        .remove_name_section(args.flag_remove_name_section)
        .typescript(typescript);
    if let Some(ref name) = args.flag_no_modules_global {
        b.no_modules_global(name);
    }

    let out_dir = match args.flag_out_dir {
        Some(ref p) => p,
        None => bail!("the `--out-dir` argument is now required"),
    };

    b.generate(out_dir)
}
