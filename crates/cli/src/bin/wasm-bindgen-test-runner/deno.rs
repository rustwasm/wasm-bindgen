use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Error};

use crate::node::{exec, SHARED_SETUP};

pub fn execute(
    module: &str,
    tmpdir: &Path,
    args: &[OsString],
    tests: &[String],
) -> Result<(), Error> {
    let mut js_to_execute = format!(
        r#"import * as wasm from "./{0}.js";

        {console_override}

        // global.__wbg_test_invoke = f => f();

        // Forward runtime arguments. These arguments are also arguments to the
        // `wasm-bindgen-test-runner` which forwards them to deno which we
        // forward to the test harness. this is basically only used for test
        // filters for now.
        cx.args(Deno.args.slice(1));

        const ok = await cx.run(tests.map(n => wasm.__wasm[n]));
        if (!ok) Deno.exit(1);

        const tests = [];
    "#,
        module,
        console_override = SHARED_SETUP,
    );

    for test in tests {
        js_to_execute.push_str(&format!("tests.push('{}')\n", test));
    }

    let js_path = tmpdir.join("run.js");
    fs::write(&js_path, js_to_execute).context("failed to write JS file")?;

    /*
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
    )*/
    exec(
        Command::new("deno")
            .arg("run")
            .arg("--allow-read")
            .arg(&js_path)
            .args(args),
    )
}
