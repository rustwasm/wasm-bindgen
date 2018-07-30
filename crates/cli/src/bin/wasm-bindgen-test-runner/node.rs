use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;

use failure::{ResultExt, Error};

pub fn execute(module: &str, tmpdir: &Path, args: &[OsString], tests: &[String])
    -> Result<(), Error>
{
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

        function main(tests) {{
            const support = require("./{0}");
            const wasm = require("./{0}_bg");

            cx = new support.Context();

            // Forward runtime arguments. These arguments are also arguments to the
            // `wasm-bindgen-test-runner` which forwards them to node which we
            // forward to the test harness. this is basically only used for test
            // filters for now.
            cx.args(process.argv.slice(2));

            if (!cx.run(tests.map(n => wasm[n])))
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
    js_to_execute.push_str("
        main(tests)
    ");

    let js_path = tmpdir.join("run.js");
    fs::write(&js_path, js_to_execute)
        .context("failed to write JS file")?;

    // Augment `NODE_PATH` so things like `require("tests/my-custom.js")` work
    // and Rust code can import from custom JS shims. This is a bit of a hack
    // and should probably be removed at some point.
    let path = env::var("NODE_PATH").unwrap_or_default();
    let mut path = env::split_paths(&path).collect::<Vec<_>>();
    path.push(env::current_dir().unwrap());
    exec(
        Command::new("node")
            .env("NODE_PATH", env::join_paths(&path).unwrap())
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
    use std::process;
    let status = cmd.status()?;
    process::exit(status.code().unwrap_or(3));
}
