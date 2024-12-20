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

use anyhow::{bail, Context};
use clap::Parser;
use clap::ValueEnum;
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

#[derive(Parser)]
#[command(name = "wasm-bindgen-test-runner", version, about, long_about = None)]
struct Cli {
    #[arg(
        index = 1,
        help = "The file to test. `cargo test` passes this argument for you."
    )]
    file: PathBuf,
    #[arg(long, conflicts_with = "ignored", help = "Run ignored tests")]
    include_ignored: bool,
    #[arg(long, conflicts_with = "include_ignored", help = "Run ignored tests")]
    ignored: bool,
    #[arg(long, help = "Exactly match filters rather than by substring")]
    exact: bool,
    #[arg(
        long,
        value_name = "FILTER",
        help = "Skip tests whose names contain FILTER (this flag can be used multiple times)"
    )]
    skip: Vec<String>,
    #[arg(long, help = "List all tests and benchmarks")]
    list: bool,
    #[arg(
        long,
        help = "don't capture `console.*()` of each task, allow printing directly"
    )]
    nocapture: bool,
    #[arg(
        long,
        value_enum,
        value_name = "terse",
        help = "Configure formatting of output"
    )]
    format: Option<FormatSetting>,
    #[arg(
        index = 2,
        value_name = "FILTER",
        help = "The FILTER string is tested against the name of all tests, and only those tests \
                whose names contain the filter are run."
    )]
    filter: Option<String>,
}

impl Cli {
    fn into_args(self, tests: &Tests) -> String {
        let include_ignored = self.include_ignored;
        let filtered = tests.filtered;

        format!(
            r#"
            // Forward runtime arguments.
            cx.include_ignored({include_ignored:?});
            cx.filtered_count({filtered});
        "#
        )
    }
}

struct Tests {
    tests: Vec<Test>,
    filtered: usize,
}

impl Tests {
    fn new() -> Self {
        Self {
            tests: Vec::new(),
            filtered: 0,
        }
    }
}

struct Test {
    name: String,
    ignored: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let shell = shell::Shell::new();

    let file_name = cli
        .file
        .file_name()
        .map(Path::new)
        .context("file to test is not a valid file, can't extract file name")?;

    // Collect all tests that the test harness is supposed to run. We assume
    // that any exported function with the prefix `__wbg_test` is a test we need
    // to execute.
    let wasm = fs::read(&cli.file).context("failed to read Wasm file")?;
    let mut wasm =
        walrus::Module::from_buffer(&wasm).context("failed to deserialize Wasm module")?;
    let mut tests = Tests::new();

    'outer: for export in wasm.exports.iter() {
        let Some(name) = export.name.strip_prefix("__wbgt_") else {
            continue;
        };
        let modifiers = name.split_once('_').expect("found invalid identifier").0;
        let test = Test {
            name: export.name.clone(),
            ignored: modifiers.contains('$'),
        };

        if let Some(filter) = &cli.filter {
            let matches = if cli.exact {
                name == *filter
            } else {
                name.contains(filter)
            };

            if !matches {
                tests.filtered += 1;
                continue;
            }
        }

        for skip in &cli.skip {
            let matches = if cli.exact {
                name == *skip
            } else {
                name.contains(skip)
            };

            if matches {
                tests.filtered += 1;
                continue 'outer;
            }
        }

        if !test.ignored && cli.ignored {
            tests.filtered += 1;
        } else {
            tests.tests.push(test);
        }
    }

    if cli.list {
        for test in tests.tests {
            println!("{}: test", test.name.split_once("::").unwrap().1);
        }

        // Returning cleanly has the strange effect of outputting
        // an additional empty line with spaces in it.
        std::process::exit(0);
    }

    let tmpdir = tempfile::tempdir()?;

    let module = "wasm-bindgen-test";

    // Right now there's a bug where if no tests are present then the
    // `wasm-bindgen-test` runtime support isn't linked in, so just bail out
    // early saying everything is ok.
    if tests.tests.is_empty() {
        println!("no tests to run!");
        return Ok(());
    }

    // Figure out if this tests is supposed to execute in node.js or a browser.
    // That's done on a per-test-binary basis with the
    // `wasm_bindgen_test_configure` macro, which emits a custom section for us
    // to read later on.

    let custom_section = wasm.customs.remove_raw("__wasm_bindgen_test_unstable");
    let no_modules = std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok();
    let test_mode = match custom_section {
        Some(section) if section.data.contains(&0x01) => TestMode::Browser { no_modules },
        Some(section) if section.data.contains(&0x02) => TestMode::DedicatedWorker { no_modules },
        Some(section) if section.data.contains(&0x03) => TestMode::SharedWorker { no_modules },
        Some(section) if section.data.contains(&0x04) => TestMode::ServiceWorker { no_modules },
        Some(section) if section.data.contains(&0x05) => TestMode::Node { no_modules },
        Some(_) => bail!("invalid __wasm_bingen_test_unstable value"),
        None => {
            let mut modes = Vec::new();
            let mut add_mode =
                |mode: TestMode| std::env::var(mode.env()).is_ok().then(|| modes.push(mode));
            add_mode(TestMode::Deno);
            add_mode(TestMode::Browser { no_modules });
            add_mode(TestMode::DedicatedWorker { no_modules });
            add_mode(TestMode::SharedWorker { no_modules });
            add_mode(TestMode::ServiceWorker { no_modules });
            add_mode(TestMode::Node { no_modules });

            match modes.len() {
                0 => TestMode::Node { no_modules: true },
                1 => modes[0],
                _ => {
                    bail!(
                        "only one test mode must be set, found: `{}`",
                        modes
                            .into_iter()
                            .map(TestMode::env)
                            .collect::<Vec<_>>()
                            .join("`, `")
                    )
                }
            }
        }
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

    let driver_timeout = env::var("WASM_BINDGEN_TEST_DRIVER_TIMEOUT")
        .map(|timeout| {
            timeout
                .parse()
                .expect("Could not parse 'WASM_BINDGEN_TEST_DRIVER_TIMEOUT'")
        })
        .unwrap_or(5);

    let browser_timeout = env::var("WASM_BINDGEN_TEST_TIMEOUT")
        .map(|timeout| {
            let timeout = timeout
                .parse()
                .expect("Could not parse 'WASM_BINDGEN_TEST_TIMEOUT'");
            println!("Set timeout to {} seconds...", timeout);
            timeout
        })
        .unwrap_or(20);

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

    let coverage = coverage_args(file_name);

    b.debug(debug)
        .input_module(module, wasm)
        .keep_debug(false)
        .emit_start(false)
        .generate(&tmpdir)
        .context("executing `wasm-bindgen` over the Wasm file")?;
    shell.clear();

    match test_mode {
        TestMode::Node { no_modules } => {
            node::execute(module, tmpdir.path(), cli, tests, !no_modules, coverage)?
        }
        TestMode::Deno => deno::execute(module, tmpdir.path(), cli, tests)?,
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
                tmpdir.path(),
                cli,
                tests,
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
            headless::run(&addr, &shell, driver_timeout, browser_timeout)?;
        }
    }
    Ok(())
}

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

    fn env(self) -> &'static str {
        match self {
            TestMode::Node { .. } => "WASM_BINDGEN_USE_NODE_EXPERIMENTAL",
            TestMode::Deno => "WASM_BINDGEN_USE_DENO",
            TestMode::Browser { .. } => "WASM_BINDGEN_USE_BROWSER",
            TestMode::DedicatedWorker { .. } => "WASM_BINDGEN_USE_DEDICATED_WORKER",
            TestMode::SharedWorker { .. } => "WASM_BINDGEN_USE_SHARED_WORKER",
            TestMode::ServiceWorker { .. } => "WASM_BINDGEN_USE_SERVICE_WORKER",
        }
    }
}

fn coverage_args(file_name: &Path) -> PathBuf {
    fn generated(file_name: &Path, prefix: &str) -> String {
        let res = format!("{prefix}{}.profraw", file_name.display());
        res
    }

    let prefix = env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_PREFIX")
        .map(|s| s.to_str().unwrap().to_string())
        .unwrap_or_default();

    match env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT") {
        Some(s) => {
            let mut buf = PathBuf::from(s);
            if buf.is_dir() {
                buf.push(generated(file_name, &prefix));
            }
            buf
        }
        None => PathBuf::from(generated(file_name, &prefix)),
    }
}

/// Possible values for the `--format` option.
#[derive(Debug, Clone, Copy, ValueEnum)]
enum FormatSetting {
    /// Display one character per test
    Terse,
}
