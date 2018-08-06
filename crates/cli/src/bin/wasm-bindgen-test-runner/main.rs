//! A "wrapper binary" used to execute wasm files as tests
//!
//! This binary is intended to be used as a "test runner" for wasm binaries,
//! being compatible with `cargo test` for the wasm target. It will
//! automatically execute `wasm-bindgen` (or the equivalent thereof) and then
//! execute either Node.js over the tests or start a server which a browser can
//! be used to run against to execute tests. In a browser mode if `CI` is in the
//! environment then it'll also attempt headless testing, spawning the server in
//! the background and then using the WebDriver protocol to execute tests.
//!
//! For more documentation about this see the `wasm-bindgen-test` crate README
//! and source code.

extern crate curl;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate parity_wasm;
extern crate rouille;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate wasm_bindgen_cli_support;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;

use failure::{ResultExt, Error};
use parity_wasm::elements::{Module, Deserialize, Section};
use wasm_bindgen_cli_support::Bindgen;

mod headless;
mod node;
mod server;
mod shell;

fn main() {
    env_logger::init();
    let err = match rmain() {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {}", err);
    for cause in err.iter_causes() {
        eprintln!("\tcaused by: {}", cause);
    }
    process::exit(1);
}

fn rmain() -> Result<(), Error> {
    let mut args = env::args_os().skip(1);
    let shell = shell::Shell::new();

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

    let module = "wasm-bindgen-test";

    // Collect all tests that the test harness is supposed to run. We assume
    // that any exported function with the prefix `__wbg_test` is a test we need
    // to execute.
    let wasm = fs::read(&wasm_file_to_test)
        .context("failed to read wasm file")?;
    let wasm = Module::deserialize(&mut &wasm[..])
        .context("failed to deserialize wasm module")?;
    let mut tests = Vec::new();
    if let Some(exports) = wasm.export_section() {
        for export in exports.entries() {
            if !export.field().starts_with("__wbg_test") {
                continue
            }
            tests.push(export.field().to_string());
        }
    }

    // Right now there's a bug where if no tests are present then the
    // `wasm-bindgen-test` runtime support isn't linked in, so just bail out
    // early saying everything is ok.
    if tests.len() == 0 {
        println!("no tests to run!");
        return Ok(())
    }

    // Figure out if this tests is supposed to execute in node.js or a browser.
    // That's done on a per-test-binary basis with the
    // `wasm_bindgen_test_configure` macro, which emits a custom section for us
    // to read later on.
    let mut node = true;
    for section in wasm.sections() {
        let custom = match section {
            Section::Custom(section) => section,
            _ => continue,
        };
        if custom.name() != "__wasm_bindgen_test_unstable" {
            continue
        }
        node = !custom.payload().contains(&0x01);
    }
    let headless = env::var("NO_HEADLESS").is_err();
    let debug = env::var("WASM_BINDGEN_NO_DEBUG").is_err();

    // Make the generated bindings available for the tests to execute against.
    shell.status("Executing bindgen...");
    let mut b = Bindgen::new();
    b.debug(debug)
        .nodejs(node)
        .input_module(module, wasm, |w| parity_wasm::serialize(w).unwrap())
        .keep_debug(false)
        .generate(&tmpdir)
        .context("executing `wasm-bindgen` over the wasm file")?;
    shell.clear();

    // If we're executing in node.js, that module will take it from here.
    if node {
        return node::execute(&module, &tmpdir, &args.collect::<Vec<_>>(), &tests)
    }

    // Otherwise we're executing in a browser. Spawn a server which serves up
    // the local generated files over an HTTP server.
    let srv = server::spawn(
        &if headless {
            "127.0.0.1:0".parse().unwrap()
        } else {
            "127.0.0.1:8000".parse().unwrap()
        },
        headless,
        &module,
        &tmpdir,
        &args.collect::<Vec<_>>(),
        &tests,
    )?;
    let addr = srv.server_addr();

    // TODO: eventually we should provide the ability to exit at some point
    // (gracefully) here, but for now this just runs forever.
    if !headless {
        println!("Interactive browsers tests are now available at http://{}", addr);
        println!("");
        println!("Note that interactive mode is enabled because `NO_HEADLESS`");
        println!("is specified in the environment of this process. Once you're");
        println!("done with testing you'll need to kill this server with");
        println!("Ctrl-C.");
        return Ok(srv.run())
    }

    thread::spawn(|| srv.run());
    headless::run(&addr, &shell)?;
    Ok(())
}
