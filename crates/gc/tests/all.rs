extern crate parity_wasm;
extern crate rayon;
extern crate tempfile;
extern crate wasm_bindgen_gc;

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use rayon::prelude::*;
use parity_wasm::elements::Module;
use tempfile::NamedTempFile;

struct Test {
    input: PathBuf,
}

fn main() {
    let mut tests = Vec::new();
    find_tests(&mut tests, "tests/wat".as_ref());

    run_tests(&tests);
}

fn find_tests(tests: &mut Vec<Test>, path: &Path) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            find_tests(tests, &path);
            continue
        }

        if path.extension().and_then(|s| s.to_str()) == Some("wat") {
            tests.push(Test {
                input: path,
            });
        }
    }
}

fn run_tests(tests: &[Test]) {
    println!("");

    let results = tests.par_iter()
        .map(|test| {
            run_test(test).map_err(|e| (test, e.to_string()))
        })
        .collect::<Vec<_>>();

    let mut bad = false;
    for result in results {
        let (test, err) = match result {
            Ok(()) => continue,
            Err(p) => p,
        };
        println!("fail: {} - {}", test.input.display(), err);
        bad = true;
    }
    if bad {
        std::process::exit(2);
    }

    println!("\nall good!");
}

fn run_test(test: &Test) -> Result<(), Box<Error>> {
    println!("test {}", test.input.display());

    let f = NamedTempFile::new()?;
    let input = fs::read_to_string(&test.input)?;
    let expected = extract_expected(&input);
    let status = Command::new("wat2wasm")
        .arg("--debug-names")
        .arg(&test.input)
        .arg("-o")
        .arg(f.path())
        .status()?;
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to run wat2wasm").into())
    }

    let wasm = fs::read(f.path())?;
    let mut module: Module = parity_wasm::deserialize_buffer(&wasm)?;
    module = match module.parse_names() {
        Ok(m) => m,
        Err((_, m)) => m,
    };
    wasm_bindgen_gc::Config::new().run(&mut module);
    let wasm = parity_wasm::serialize(module)?;
    fs::write(f.path(), wasm)?;

    let status = Command::new("wasm2wat")
        .arg(&f.path())
        .stderr(Stdio::inherit())
        .output()?;
    if !status.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to run wasm2wat").into())
    }
    let actual = String::from_utf8(status.stdout)?;
    let actual = actual.trim();

    if env::var("BLESS_TESTS").is_ok() {
        fs::write(&test.input, generate_blesssed(&input, &actual))?;
    } else {
        if actual != expected {
            println!("{:?} {:?}", actual, expected);
            return Err(io::Error::new(io::ErrorKind::Other,
                                      "test failed").into())
        }
    }

    Ok(())
}

fn extract_expected(input: &str) -> String {
    input.lines()
        .filter(|l| l.starts_with(";; "))
        .skip_while(|l| !l.contains("STDOUT"))
        .skip(1)
        .take_while(|l| !l.contains("STDOUT"))
        .map(|l| &l[3..])
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_blesssed(input: &str, actual: &str) -> String {
    let mut input = input.lines()
        .filter(|l| !l.starts_with(";;"))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();
    input.push_str("\n\n");
    input.push_str(";; STDOUT (update this section with `BLESS_TESTS=1` while running tests)\n");
    for line in actual.lines() {
        input.push_str(";; ");
        input.push_str(line);
        input.push_str("\n");
    }
    input.push_str(";; STDOUT\n");
    return input
}
