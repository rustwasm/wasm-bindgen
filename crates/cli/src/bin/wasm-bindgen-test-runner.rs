#[macro_use]
extern crate failure;
extern crate wasm_bindgen_cli_support;
extern crate parity_wasm;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

use failure::{ResultExt, Error};
use parity_wasm::elements::{Module, Deserialize};
use wasm_bindgen_cli_support::Bindgen;

fn main() {
    let err = match rmain() {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {}", err);
    for cause in err.causes().skip(1) {
        eprintln!("\tcaused by: {}", cause);
    }
    process::exit(1);
}

fn rmain() -> Result<(), Error> {
    let mut args = env::args_os().skip(1);

    // Currently no flags are supported, and assume there's only one argument
    // which is the wasm file to test. This'll want to improve over time!
    let wasm_file_to_test = match args.next() {
        Some(file) => PathBuf::from(file),
        None => bail!("must have a file to test as first argument"),
    };

    // Assume a cargo-like directory layout and generate output at
    // `target/wasm32-unknown-unknown/wbg-tmp/...`
    let tmpdir = wasm_file_to_test.parent() // chop off file name
        .and_then(|p| p.parent())           // chop off `deps`
        .and_then(|p| p.parent())           // chop off `debug`
        .map(|p| p.join("wbg-tmp"))
        .ok_or_else(|| {
            format_err!("file to test doesn't follow the expected Cargo conventions")
        })?;

    // Make sure there's no stale state from before
    drop(fs::remove_dir_all(&tmpdir));
    fs::create_dir(&tmpdir)
        .context("creating temporary directory")?;

    let module = wasm_file_to_test.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format_err!("invalid filename passed in"))?;

    let mut js_to_execute = format!(r#"
        const {{ exit }} = require('process');

        let cx = null;

        // override `console.log` and `console.error` before we import tests to
        // ensure they're bound correctly in wasm. This'll allow us to intercept
        // all these calls and capture the output of tests
        const prev_log = console.log;
        console.log = function() {{
            if (cx === null)  {{
                prev_log.apply(null, arguments);
            }} else {{
                cx.console_log(prev_log, arguments);
            }}
        }};
        const prev_error = console.error;
        console.error = function() {{
            if (cx === null) {{
                prev_error.apply(null, arguments);
            }} else {{
                cx.console_error(prev_error, arguments);
            }}
        }};

        const support = require("./{0}");
        const wasm = require("./{0}_bg");

        // Hack for now to support 0 tests in a binary. This should be done
        // better...
        if (support.Context === undefined)
            process.exit(0);

        cx = new support.Context();

        // Forward runtime arguments. These arguments are also arguments to the
        // `wasm-bindgen-test-runner` which forwards them to node which we
        // forward to the test harness. this is basically only used for test
        // filters for now.
        cx.args(process.argv.slice(2));

        const tests = [];
    "#,
        module
    );

    // Collect all tests that the test harness is supposed to run. We assume
    // that any exported function with the prefix `__wbg_test` is a test we need
    // to execute.
    //
    // Note that we're collecting *JS objects* that represent the functions to
    // execute, and then those objects are passed into wasm for it to execute
    // when it sees fit.
    let wasm = fs::read(&wasm_file_to_test)
        .context("failed to read wasm file")?;
    let wasm = Module::deserialize(&mut &wasm[..])
        .context("failed to deserialize wasm module")?;
    if let Some(exports) = wasm.export_section() {
        for export in exports.entries() {
            if !export.field().starts_with("__wbg_test") {
                continue
            }
            js_to_execute.push_str(&format!("tests.push(wasm.{})\n", export.field()));
        }
    }

    // And as a final addendum, exit with a nonzero code if any tests fail.
    js_to_execute.push_str("if (!cx.run(tests)) exit(1);\n");

    // For now unconditionally generate wasm-bindgen code tailored for node.js,
    // but eventually we'll want more options here for browsers!
    let mut b = Bindgen::new();
    b.debug(true)
        .nodejs(true)
        .input_module(module, wasm, |m| parity_wasm::serialize(m).unwrap())
        .keep_debug(false)
        .generate(&tmpdir)
        .context("executing `wasm-bindgen` over the wasm file")?;

    let js_path = tmpdir.join("run.js");
    fs::write(&js_path, js_to_execute)
        .context("failed to write JS file")?;

    // Last but not least, execute `node`! Add an entry to `NODE_PATH` for the
    // project root to hopefully pick up `node_modules` and other local imports.
    let path = env::var_os("NODE_PATH").unwrap_or_default();
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    paths.push(env::current_dir().unwrap());
    exec(
        Command::new("node")
            .env("NODE_PATH", env::join_paths(&paths).unwrap())
            .arg(&js_path)
            .args(args)
    )
}

#[cfg(unix)]
fn exec(cmd: &mut Command) -> Result<(), Error> {
    use std::os::unix::prelude::*;
    Err(Error::from(cmd.exec()).context("failed to execute `node`").into())
}

#[cfg(windows)]
fn exec(cmd: &mut Command) -> Result<(), Error> {
    let status = cmd.status()?;
    process::exit(status.code().unwrap_or(3));
}
