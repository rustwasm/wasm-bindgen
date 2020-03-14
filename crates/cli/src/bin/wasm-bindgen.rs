use anyhow::{bail, Error};
use docopt::Docopt;
use serde::Deserialize;
use std::path::PathBuf;
use std::process;
use wasm_bindgen_cli_support::{Bindgen, EncodeInto};

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
    -h --help                    Show this screen.
    --out-dir DIR                Output directory
    --out-name VAR               Set a custom output filename (Without extension. Defaults to crate name)
    --target TARGET              What type of output to generate, valid
                                 values are [web, bundler, nodejs, no-modules],
                                 and the default is [bundler]
    --no-modules-global VAR      Name of the global variable to initialize
    --browser                    Hint that JS should only be compatible with a browser
    --typescript                 Output a TypeScript definition file (on by default)
    --no-typescript              Don't emit a *.d.ts file
    --omit-imports               Don't emit imports in generated JavaScript
    --debug                      Include otherwise-extraneous debug checks in output
    --no-demangle                Don't demangle Rust symbol names
    --keep-debug                 Keep debug sections in wasm files
    --remove-name-section        Remove the debugging `name` section of the file
    --remove-producers-section   Remove the telemetry `producers` section
    --encode-into MODE           Whether or not to use TextEncoder#encodeInto,
                                 valid values are [test, always, never]
    --nodejs                     Deprecated, use `--target nodejs`
    --web                        Deprecated, use `--target web`
    --no-modules                 Deprecated, use `--target no-modules`
    -V --version                 Print the version number of wasm-bindgen
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_nodejs: bool,
    flag_browser: bool,
    flag_web: bool,
    flag_no_modules: bool,
    flag_typescript: bool,
    flag_no_typescript: bool,
    flag_omit_imports: bool,
    flag_out_dir: Option<PathBuf>,
    flag_out_name: Option<String>,
    flag_debug: bool,
    flag_version: bool,
    flag_no_demangle: bool,
    flag_no_modules_global: Option<String>,
    flag_remove_name_section: bool,
    flag_remove_producers_section: bool,
    flag_keep_debug: bool,
    flag_encode_into: Option<String>,
    flag_target: Option<String>,
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
    eprintln!("error: {:?}", err);
    process::exit(1);
}

fn rmain(args: &Args) -> Result<(), Error> {
    let input = match args.arg_input {
        Some(ref s) => s,
        None => bail!("input file expected"),
    };

    let typescript = args.flag_typescript || !args.flag_no_typescript;

    let mut b = Bindgen::new();
    if let Some(name) = &args.flag_target {
        match name.as_str() {
            "bundler" => b.bundler(true)?,
            "web" => b.web(true)?,
            "no-modules" => b.no_modules(true)?,
            "nodejs" => b.nodejs(true)?,
            s => bail!("invalid encode-into mode: `{}`", s),
        };
    }
    b.input_path(input)
        .nodejs(args.flag_nodejs)?
        .web(args.flag_web)?
        .browser(args.flag_browser)?
        .no_modules(args.flag_no_modules)?
        .debug(args.flag_debug)
        .demangle(!args.flag_no_demangle)
        .keep_debug(args.flag_keep_debug)
        .remove_name_section(args.flag_remove_name_section)
        .remove_producers_section(args.flag_remove_producers_section)
        .typescript(typescript)
        .omit_imports(args.flag_omit_imports);
    if let Some(ref name) = args.flag_no_modules_global {
        b.no_modules_global(name)?;
    }
    if let Some(ref name) = args.flag_out_name {
        b.out_name(name);
    }
    if let Some(mode) = &args.flag_encode_into {
        match mode.as_str() {
            "test" => b.encode_into(EncodeInto::Test),
            "always" => b.encode_into(EncodeInto::Always),
            "never" => b.encode_into(EncodeInto::Never),
            s => bail!("invalid encode-into mode: `{}`", s),
        };
    }

    let out_dir = match args.flag_out_dir {
        Some(ref p) => p,
        None => bail!("the `--out-dir` argument is now required"),
    };

    b.generate(out_dir)
}
