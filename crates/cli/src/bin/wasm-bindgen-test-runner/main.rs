//! A "wrapper binary" used to execute Wasm files as tests
//!
//! This binary is intended to be used as a "test runner" for Wasm binaries,
//! being compatible with `cargo test` for the Wasm target. It will
//! automatically execute `wasm-bindgen` (or the equivalent thereof) and then
//! execute either Node.js over the tests or start a server which a browser can
//! be used to run against to execute tests. In a browser mode if `CI` is in the
//! environment then it'll also attempt headless testing, spawning the server in
//! the background and then using the WebDriver protocol to execute tests.
//!
//! For more documentation about this see the `wasm-bindgen-test` crate README
//! and source code.

use anyhow::{anyhow, bail, Context};
use log::error;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use wasm_bindgen_cli_support::Bindgen;

mod deno;
mod headless;
mod node;
mod server;
mod shell;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TestMode {
    Node { no_modules: bool },
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
            Self::Deno => true,
            Self::Browser { no_modules }
            | Self::Node { no_modules }
            | Self::DedicatedWorker { no_modules }
            | Self::SharedWorker { no_modules }
            | Self::ServiceWorker { no_modules } => no_modules,
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

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut args = env::args_os().skip(1);
    let shell = shell::Shell::new();

    // Currently no flags are supported, and assume there's only one argument
    // which is the Wasm file to test. This'll want to improve over time!
    let wasm_file_to_test = match args.next() {
        Some(file) => PathBuf::from(file),
        None => bail!("must have a file to test as first argument"),
    };

    let file_name = wasm_file_to_test
        .file_name()
        .and_then(|s| s.to_str())
        .context("file to test is not a valid file, can't extract file name")?;

    // wasm_file_to_test may be
    // - a cargo-like directory layout and generate output at
    //      `target/wasm32-unknown-unknown/...`
    // - a tmp directory, generated by rustdoc
    // we would like a directory we have write access to. if we assume cargo-like directories,
    // we end up with the path `/wbg-out`
    let wasm_file_str = wasm_file_to_test.to_string_lossy();
    let tmpdir =
        if wasm_file_str.starts_with("/tmp/rustdoc") || wasm_file_str.starts_with("/var/folders") {
            wasm_file_to_test.parent() // chop off the file name and give us the /tmp/rustdoc<hash> directory
        } else {
            wasm_file_to_test
                .parent() // chop off file name
                .and_then(|p| p.parent()) // chop off `deps`
                .and_then(|p| p.parent()) // chop off `debug`
        }
        .map(|p| p.join(format!("wbg-tmp-{}", file_name)))
        .ok_or_else(|| anyhow!("file to test doesn't follow the expected Cargo conventions"))?;

    // Make sure there's no stale state from before
    drop(fs::remove_dir_all(&tmpdir));
    fs::create_dir(&tmpdir).context("creating temporary directory")?;
    let _guard = TmpDirDeleteGuard(tmpdir.clone());

    let module = "wasm-bindgen-test";

    // Collect all tests that the test harness is supposed to run. We assume
    // that any exported function with the prefix `__wbg_test` is a test we need
    // to execute.
    let wasm = fs::read(&wasm_file_to_test).context("failed to read Wasm file")?;
    let mut wasm =
        walrus::Module::from_buffer(&wasm).context("failed to deserialize Wasm module")?;
    let mut tests = Vec::new();

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
        Some(section) if section.data.contains(&0x05) => TestMode::Node {
            no_modules: std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok(),
        },
        Some(_) => bail!("invalid __wasm_bingen_test_unstable value"),
        None if std::env::var("WASM_BINDGEN_USE_DENO").is_ok() => TestMode::Deno,
        None => TestMode::Node { no_modules: true },
    };

    let headless = env::var("NO_HEADLESS").is_err();
    let debug = env::var("WASM_BINDGEN_NO_DEBUG").is_err();

    // Gracefully handle requests to execute only node or only web tests.
    let node = matches!(test_mode, TestMode::Node { .. });

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

    // Make the generated bindings available for the tests to execute against.
    shell.status("Executing bindgen...");
    let mut b = Bindgen::new();
    match test_mode {
        TestMode::Node { no_modules: true } => b.nodejs(true)?,
        TestMode::Node { no_modules: false } => b.nodejs_module(true)?,
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

    let coverage = coverage_args(&tmpdir);

    b.debug(debug)
        .input_module(module, wasm)
        .emit_start(false)
        .generate(&tmpdir)
        .context("executing `wasm-bindgen` over the Wasm file")?;
    shell.clear();

    let args: Vec<_> = args.collect();

    match test_mode {
        TestMode::Node { no_modules } => {
            node::execute(module, &tmpdir, &args, &tests, !no_modules)?
        }
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
                coverage,
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

fn coverage_args(tmpdir: &Path) -> PathBuf {
    fn generated(tmpdir: &Path, prefix: &str) -> String {
        let res = format!(
            "{prefix}{}.profraw",
            tmpdir.file_name().and_then(|s| s.to_str()).unwrap()
        );
        res
    }

    let prefix = env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_PREFIX")
        .map(|s| s.to_str().unwrap().to_string())
        .unwrap_or_default();

    match env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT") {
        Some(s) => {
            let mut buf = PathBuf::from(s);
            if buf.is_dir() {
                buf.push(generated(tmpdir, &prefix));
            }
            buf
        }
        None => PathBuf::from(generated(tmpdir, &prefix)),
    }
}
