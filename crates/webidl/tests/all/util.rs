use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process;
use std::sync::{Once, ONCE_INIT};

use diff;
use env_logger;
use wb_webidl;

fn rustfmt<S: Into<String>>(source: S) -> (String, String) {
    let source = source.into();

    static CHECK_RUSTFMT: Once = ONCE_INIT;

    CHECK_RUSTFMT.call_once(|| {
        let have_working_rustfmt = process::Command::new("rustup")
            .args(&["run", "nightly", "rustfmt", "--version"])
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .status()
            .ok()
            .map_or(false, |status| status.success());

        if !have_working_rustfmt {
            panic!(
                "
The latest `rustfmt` is required to run the `wasm-bindgen` test suite. Install
`rustfmt` with:

    $ rustup component add rustfmt-preview --toolchain nightly
"
            );
        }
    });

    let mut child = process::Command::new("rustup")
        .args(&[
            "run",
            "nightly",
            "rustfmt",
            "--config-path",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/rustfmt.toml"),
        ])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .expect("should spawn `rustup run nightly rustfmt`");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || stdin.write_all(source.as_bytes()));

    // Read stderr on a new thread for similar reasons.
    let stderr_handle = ::std::thread::spawn(move || {
        let mut output = vec![];
        io::copy(&mut stderr, &mut output).map(|_| String::from_utf8_lossy(&output).to_string())
    });

    let mut output = vec![];
    io::copy(&mut stdout, &mut output).expect("Should copy stdout into vec OK");

    // Ignore actual rustfmt status because it is often non-zero for trivial
    // things.
    let _ = child.wait().expect("should wait on rustfmt child OK");

    stdin_handle
        .join()
        .expect("writer thread should not have panicked")
        .expect("should have written to child rustfmt's stdin OK");

    let formatted = String::from_utf8(output).expect("rustfmt should only emit valid utf-8");

    let stderr = stderr_handle
        .join()
        .expect("stderr reader thread should not have panicked")
        .expect("should have read child rustfmt's stderr OK");

    (formatted, stderr)
}

fn strip_wasm_bindgen_generated(source: &str) -> String {
    let lines: Vec<_> = source
        .lines()
        .filter(|l| !l.contains("__WASM_BINDGEN_GENERATED"))
        .collect();
    lines.join("\n")
}

pub fn assert_compile(webidl: &str, expected: &str, expected_file: &str) {
    static INIT_ENV_LOGGER: Once = ONCE_INIT;
    INIT_ENV_LOGGER.call_once(|| {
        env_logger::init();
    });

    let actual = wb_webidl::compile(webidl).expect("should compile the webidl source OK");

    let (actual_orig, actual_stderr) = rustfmt(actual);
    let (expected, expected_stderr) = rustfmt(expected);

    let actual = strip_wasm_bindgen_generated(&actual_orig);
    let expected = strip_wasm_bindgen_generated(&expected);

    if expected == actual {
        return;
    }

    if env::var("UPDATE_EXPECTED").is_ok() {
        File::create(expected_file)
            .unwrap()
            .write_all(actual_orig.as_bytes())
            .unwrap();
        return
    }

    eprintln!("rustfmt(expected) stderr:");
    eprintln!("{}", expected_stderr);
    eprintln!();

    eprintln!("rustfmt(actual) stderr:");
    eprintln!("{}", actual_stderr);
    eprintln!();

    eprintln!(
        "assert_compile failed: actual compiled output and expected compiled output do not match:"
    );
    eprintln!("--- expected");
    eprintln!("+++ actual");
    for d in diff::lines(&expected, &actual) {
        match d {
            diff::Result::Left(l) => eprintln!("-{}", l),
            diff::Result::Right(r) => eprintln!("+{}", r),
            diff::Result::Both(b, _) => eprintln!(" {}", b),
        }
    }

    panic!()
}

macro_rules! assert_compile {
    ($test_name:ident) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let webidl_source = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/",
                stringify!($test_name),
                ".webidl"
            ));
            let expected_output = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/expected/",
                stringify!($test_name),
                ".rs"
            ));
            let expected_file = concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/expected/",
                stringify!($test_name),
                ".rs"
            );
            $crate::assert_compile(webidl_source, expected_output, expected_file);
        }
    };
}
