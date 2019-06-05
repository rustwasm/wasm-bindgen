use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;

use failure::{Error, ResultExt};

pub fn execute(
    module: &str,
    tmpdir: &Path,
    args: &[OsString],
    tests: &[String],
) -> Result<(), Error> {
    let mut js_to_execute = format!(
        r#"
        const {{ exit }} = require('process');

        const handlers = {{}};

        const wrap = method => {{
            const og = console[method];
            const on_method = `on_console_${{method}}`;
            console[method] = function (...args) {{
                og.apply(this, args);
                if (handlers[on_method]) {{
                    handlers[on_method](args);
                }}
            }};
        }};

        // override `console.log` and `console.error` etc... before we import tests to
        // ensure they're bound correctly in wasm. This'll allow us to intercept
        // all these calls and capture the output of tests
        wrap("debug");
        wrap("log");
        wrap("info");
        wrap("warn");
        wrap("error");

        global.__wbg_test_invoke = f => f();

        async function main(tests) {{
            const support = require("./{0}");
            const wasm = require("./{0}_bg");

            cx = new support.WasmBindgenTestContext();
            handlers.on_console_debug = support.__wbgtest_console_debug;
            handlers.on_console_log = support.__wbgtest_console_log;
            handlers.on_console_info = support.__wbgtest_console_info;
            handlers.on_console_warn = support.__wbgtest_console_warn;
            handlers.on_console_error = support.__wbgtest_console_error;

            // Forward runtime arguments. These arguments are also arguments to the
            // `wasm-bindgen-test-runner` which forwards them to node which we
            // forward to the test harness. this is basically only used for test
            // filters for now.
            cx.args(process.argv.slice(2));

            const ok = await cx.run(tests.map(n => wasm[n]));
            if (!ok)
                exit(1);
        }}

        const tests = [];
    "#,
        module
    );

    // Note that we're collecting *JS objects* that represent the functions to
    // execute, and then those objects are passed into wasm for it to execute
    // when it sees fit.
    for test in tests {
        js_to_execute.push_str(&format!("tests.push('{}')\n", test));
    }
    // And as a final addendum, exit with a nonzero code if any tests fail.
    js_to_execute.push_str(
        "
        main(tests)
            .catch(e => {
                console.error(e);
                exit(1);
            });
    ",
    );

    let js_path = tmpdir.join("run.js");
    fs::write(&js_path, js_to_execute).context("failed to write JS file")?;

    // Augment `NODE_PATH` so things like `require("tests/my-custom.js")` work
    // and Rust code can import from custom JS shims. This is a bit of a hack
    // and should probably be removed at some point.
    let path = env::var("NODE_PATH").unwrap_or_default();
    let mut path = env::split_paths(&path).collect::<Vec<_>>();
    path.push(env::current_dir().unwrap());
    path.push(tmpdir.to_path_buf());
    let extra_node_args = env::var("NODE_ARGS")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    exec(
        Command::new("node")
            .env("NODE_PATH", env::join_paths(&path).unwrap())
            .args(&extra_node_args)
            .arg(&js_path)
            .args(args),
    )
}

#[cfg(unix)]
fn exec(cmd: &mut Command) -> Result<(), Error> {
    use std::os::unix::prelude::*;
    Err(Error::from(cmd.exec())
        .context("failed to execute `node`")
        .into())
}

#[cfg(windows)]
fn exec(cmd: &mut Command) -> Result<(), Error> {
    use std::process;
    let status = cmd.status()?;
    process::exit(status.code().unwrap_or(3));
}
