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

use anyhow::{anyhow, bail, Context, Result};
use auroka_common_concurrency_filesystem_resource_coordinator::ResourceCoordinator;
use commands::list;
use commands::version;
use docopt::Docopt;
use log::error;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;
use wasm_bindgen_cli_support::Bindgen;

mod commands;
mod deno;
mod headless;
mod lock;
mod node;
mod server;
mod shell;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TestMode {
    Node,
    Deno,
    Browser { no_modules: bool },
    DedicatedWorker { no_modules: bool },
    SharedWorker { no_modules: bool },
    ServiceWorker { no_modules: bool },
}

impl TestMode {
    fn is_worker(self) -> bool {
        matches!(
            self,
            Self::DedicatedWorker { .. } | Self::SharedWorker { .. } | Self::ServiceWorker { .. }
        )
    }

    fn no_modules(self) -> bool {
        match self {
            Self::Node => true,
            Self::Deno => true,
            Self::Browser { no_modules }
            | Self::DedicatedWorker { no_modules }
            | Self::SharedWorker { no_modules }
            | Self::ServiceWorker { no_modules } => no_modules,
        }
    }

    fn summary(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Deno => "deno",
            Self::Browser { .. }
            | Self::DedicatedWorker { .. }
            | Self::SharedWorker { .. }
            | Self::ServiceWorker { .. } => {
                if self.no_modules() {
                    "no_modules"
                } else {
                    "web"
                }
            }
        }
    }
}

struct TmpDirDeleteGuard(PathBuf);

impl Drop for TmpDirDeleteGuard {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            error!("failed to remove temporary directory: {}", e);
        }
    }
}

const USAGE: &str = "
Execute all wasm bindgen unit and integration tests and build examples of a local package

Usage:
    wasm-bindgen-test-runner [options] <input> [<testname>] [--include-ignored] [(--skip PATTERN)...] [--nocapture]
    wasm-bindgen-test-runner [options] <input> <testname> [--nocapture] --exact
    wasm-bindgen-test-runner [options] <input> --list [--format FORMAT] [--ignored]
    wasm-bindgen-test-runner -h | --help
    wasm-bindgen-test-runner -V | --version

Options:
    -h, --help         Show this screen.
    -V, --version      Print the version number of wasm-bindgen-test-runner

    <input>            The wasm file to test
    <testname>         If specified, only executes the tests containing <testname> in their names

Arguments:
    --include-ignored  Include ignored tests in the test run
    --skip PATTERN     Skip tests whose names match the given pattern
    --nocapture        Disables the tests output capture
    --exact            Run only the test with the exact name

    --list             List all tests that would be run
    --format FORMAT    Format of the tests listing output, valid values are [terse, json]
    --ignored          Restricts the listing to only consider the ignored tests

Additional documentation: https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: Option<PathBuf>,
    arg_testname: Option<String>,
    flag_exact: bool,
    flag_format: Option<String>,
    flag_include_ignored: bool,
    flag_ignored: bool,
    flag_list: bool,
    flag_nocapture: bool,
    flag_pattern: Vec<String>,
    flag_version: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = env::args_os().skip(2);
    let args_: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args_.flag_version {
        return version();
    }

    let wasm_file_to_test: PathBuf = if let Some(input) = args_.arg_input {
        input
    } else {
        bail!("must have a file to test as first argument");
    };

    let file_name = wasm_file_to_test
        .file_name()
        .and_then(|s| s.to_str())
        .context("file to test is not a valid file, can't extract file name")?;

    let wasm = fs::read(&wasm_file_to_test).context("failed to read wasm file")?;
    let mut wasm =
        walrus::Module::from_buffer(&wasm).context("failed to deserialize wasm module")?;

    if args_.flag_list {
        return list(&wasm, args_.flag_ignored);
    }

    let mut tests = Vec::new();
    // Collect all tests that the test harness is supposed to run. We assume
    // that any exported function with the prefix `__wbg_test` is a test we need
    // to execute.
    for export in wasm.exports.iter() {
        if !export.name.starts_with("__wbgt_") {
            continue;
        }
        tests.push(export.name.to_string());
    }

    // Right now there's a bug where if no tests are present then the
    // `wasm-bindgen-test` runtime support isn't linked in, so just bail out
    // early saying everything is ok.
    if tests.is_empty() {
        println!("no tests to run!");
        return Ok(());
    }

    // Figure out if this tests is supposed to execute in node.js or a browser.
    // That's done on a per-test-binary basis with the
    // `wasm_bindgen_test_configure` macro, which emits a custom section for us
    // to read later on.

    let custom_section = wasm.customs.remove_raw("__wasm_bindgen_test_unstable");
    let test_mode = match custom_section {
        Some(section) if section.data.contains(&0x01) => TestMode::Browser {
            no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
        },
        Some(section) if section.data.contains(&0x02) => TestMode::DedicatedWorker {
            no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
        },
        Some(section) if section.data.contains(&0x03) => TestMode::SharedWorker {
            no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
        },
        Some(section) if section.data.contains(&0x04) => TestMode::ServiceWorker {
            no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
        },
        Some(_) => bail!("invalid __wasm_bingen_test_unstable value"),
        None if std::env::var("WASM_BINDGEN_USE_DENO").is_ok() => TestMode::Deno,
        None if std::env::var("WASM_BINDGEN_USE_BROWSER").is_ok() => TestMode::Browser {
            no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
        },
        None if std::env::var("WASM_BINDGEN_USE_DEDICATED_WORKER").is_ok() => {
            TestMode::DedicatedWorker {
                no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
            }
        }
        None if std::env::var("WASM_BINDGEN_USE_SERVICE_WORKER").is_ok() => {
            TestMode::DedicatedWorker {
                no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
            }
        }
        None if std::env::var("WASM_BINDGEN_USE_SHARED_WORKER").is_ok() => {
            TestMode::DedicatedWorker {
                no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
            }
        }
        None => TestMode::Node,
    };

    let headless = env::var("NO_HEADLESS").is_err();
    let debug = env::var("WASM_BINDGEN_NO_DEBUG").is_err();

    // Gracefully handle requests to execute only node or only web tests.
    let node = test_mode == TestMode::Node;

    if env::var_os("WASM_BINDGEN_TEST_ONLY_NODE").is_some() && !node {
        println!(
            "this test suite is only configured to run in a browser, \
             but we're only testing node.js tests so skipping"
        );
        return Ok(());
    }
    if env::var_os("WASM_BINDGEN_TEST_ONLY_WEB").is_some() && node {
        println!(
            "\
    This test suite is only configured to run in node.js, but we're only running
    browser tests so skipping. If you'd like to run the tests in a browser
    include this in your crate when testing:

        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    You'll likely want to put that in a `#[cfg(test)]` module or at the top of an
    integration test.\
    "
        );
        return Ok(());
    }

    let module = "wasm-bindgen-test";

    // wasm_file_to_test may be
    // - a cargo-like directory layout and generate output at
    //      `target/wasm32-unknown-unknown/...`
    // - a tmp directory, generated by rustdoc
    // we would like a directory we have write access to. if we assume cargo-like directories,
    // we end up with the path `/wbg-out`
    let wasm_file_str = wasm_file_to_test.to_string_lossy();
    let mut tmp = format!("wbg-tmp-{}-{}", file_name, test_mode.summary());
    if debug {
        tmp += "-debug";
    }
    if std::env::var("WASM_BINDGEN_SPLIT_LINKED_MODULES").is_ok() {
        tmp += "-split";
    }
    let tmpdir =
        if wasm_file_str.starts_with("/tmp/rustdoc") || wasm_file_str.starts_with("/var/folders") {
            wasm_file_to_test.parent() // chop off the file name and give us the /tmp/rustdoc<hash> directory
        } else {
            wasm_file_to_test
                .parent() // chop off file name
                .and_then(|p| p.parent()) // chop off `deps`
                .and_then(|p| p.parent()) // chop off `debug`
        }
        .map(|p| p.join(tmp.clone()))
        .ok_or_else(|| anyhow!("file to test doesn't follow the expected Cargo conventions"))?;

    let shell = shell::Shell::new();

    let mut resource_coordinator = ResourceCoordinator::try_new(tmp)?;

    resource_coordinator.initialize({
        let shell = shell.clone();
        let tmpdir = tmpdir.clone();
        move || -> Result<()> {
            // Make sure there's no stale state from before
            drop(fs::remove_dir_all(&tmpdir));
            fs::create_dir(&tmpdir).context("creating temporary directory")?;

            // Make the generated bindings available for the tests to execute against.
            shell.status("Executing bindgen...");

            let mut b = Bindgen::new();
            match test_mode {
                TestMode::Node => b.nodejs(true)?,
                TestMode::Deno => b.deno(true)?,
                TestMode::Browser { .. }
                | TestMode::DedicatedWorker { .. }
                | TestMode::SharedWorker { .. }
                | TestMode::ServiceWorker { .. } => {
                    if test_mode.no_modules() {
                        b.no_modules(true)?
                    } else {
                        b.web(true)?
                    }
                }
            };

            if std::env::var("WASM_BINDGEN_SPLIT_LINKED_MODULES").is_ok() {
                b.split_linked_modules(true);
            }

            b.debug(debug)
                .input_module(module, wasm)
                .keep_debug(false)
                .emit_start(false)
                .generate(&tmpdir)
                .context("executing `wasm-bindgen` over the wasm file")?;

            shell.clear();

            Ok(())
        }
    });

    resource_coordinator.finalize({
        let tmpdir = tmpdir.clone();
        move || {
            if fs::exists(&tmpdir).unwrap() {
                if let Err(e) = fs::remove_dir_all(&tmpdir) {
                    error!("failed to remove temporary directory: {}", e);
                }
            }
        }
    });

    resource_coordinator.enter()?;

    let timeout = env::var("WASM_BINDGEN_TEST_TIMEOUT")
        .map(|timeout| {
            timeout
                .parse()
                .expect("Could not parse 'WASM_BINDGEN_TEST_TIMEOUT'")
        })
        .unwrap_or(20);

    if debug {
        println!("Set timeout to {} seconds...", timeout);
    }

    let args: Vec<_> = args.collect();

    match test_mode {
        TestMode::Node => node::execute(module, &tmpdir, &args, &tests)?,
        TestMode::Deno => deno::execute(module, &tmpdir, &args, &tests)?,
        TestMode::Browser { .. }
        | TestMode::DedicatedWorker { .. }
        | TestMode::SharedWorker { .. }
        | TestMode::ServiceWorker { .. } => {
            let srv = server::spawn(
                &if headless {
                    "127.0.0.1:0".parse().unwrap()
                } else if let Ok(address) = std::env::var("WASM_BINDGEN_TEST_ADDRESS") {
                    address.parse().unwrap()
                } else {
                    "127.0.0.1:8000".parse().unwrap()
                },
                headless,
                module,
                &tmpdir,
                &args,
                &tests,
                test_mode,
                std::env::var("WASM_BINDGEN_TEST_NO_ORIGIN_ISOLATION").is_err(),
            )
            .context("failed to spawn server")?;
            let addr = srv.server_addr();

            // TODO: eventually we should provide the ability to exit at some point
            // (gracefully) here, but for now this just runs forever.
            if !headless {
                println!(
                    "Interactive browsers tests are now available at http://{}",
                    addr
                );
                println!();
                println!("Note that interactive mode is enabled because `NO_HEADLESS`");
                println!("is specified in the environment of this process. Once you're");
                println!("done with testing you'll need to kill this server with");
                println!("Ctrl-C.");
                srv.run();
                return Ok(());
            }

            thread::spawn(|| srv.run());
            headless::run(&addr, &shell, timeout)?;
        }
    }
    Ok(())
}
