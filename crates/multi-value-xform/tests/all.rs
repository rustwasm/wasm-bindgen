//! A small test framework to execute a test function over all files in a
//! directory.
//!
//! Each file in the directory has its own `CHECK-ALL` annotation indicating the
//! expected output of the test. That can be automatically updated with
//! `BLESS=1` in the environment. Otherwise the test are checked against the
//! listed expectation.

use anyhow::{bail, Context, Result};
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use wast::parser::{Parse, Parser};

fn main() {
    run("tests".as_ref(), runtest);
}

fn runtest(test: &Test) -> Result<String> {
    let wasm = wat::parse_file(&test.file)?;
    let mut walrus = walrus::Module::from_buffer(&wasm)?;
    let mut exports = Vec::new();
    let mut xforms = Vec::new();
    for directive in test.directives.iter() {
        let export = walrus
            .exports
            .iter()
            .find(|e| e.name == directive.name)
            .unwrap();
        let id = match export.item {
            walrus::ExportItem::Function(id) => id,
            _ => panic!("must be function export"),
        };
        exports.push(export.id());
        xforms.push((id, 0, directive.tys.clone()));
    }
    let memory = walrus.memories.iter().next().unwrap().id();
    let stack_pointer = walrus.globals.iter().next().unwrap().id();
    let ret = wasm_bindgen_multi_value_xform::run(&mut walrus, memory, stack_pointer, &xforms)?;
    for (export, id) in exports.into_iter().zip(ret) {
        walrus.exports.get_mut(export).item = walrus::ExportItem::Function(id);
    }
    walrus::passes::gc::run(&mut walrus);
    let printed = wasmprinter::print_bytes(&walrus.emit_wasm())?;
    Ok(printed)
}

fn run(dir: &Path, run: fn(&Test) -> Result<String>) {
    let mut tests = Vec::new();
    find_tests(dir, &mut tests);
    let filter = std::env::args().nth(1);

    let bless = env::var("BLESS").is_ok();
    let tests = tests
        .iter()
        .filter(|test| {
            if let Some(filter) = &filter {
                if let Some(s) = test.file_name().and_then(|s| s.to_str()) {
                    if !s.contains(filter) {
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>();

    println!("\nrunning {} tests\n", tests.len());

    let errors = tests
        .par_iter()
        .filter_map(|test| run_test(test, bless, run).err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for msg in errors.iter() {
            eprintln!("error: {:?}", msg);
        }

        panic!("{} tests failed", errors.len())
    }

    println!("test result: ok. {} passed\n", tests.len());
}

fn run_test(test: &Path, bless: bool, run: fn(&Test) -> anyhow::Result<String>) -> Result<()> {
    (|| -> Result<_> {
        let expected = Test::from_file(test)?;
        let actual = run(&expected)?;
        expected.check(&actual, bless)?;
        Ok(())
    })()
    .context(format!("test failed - {}", test.display()))?;
    Ok(())
}

fn find_tests(path: &Path, tests: &mut Vec<PathBuf>) {
    for f in path.read_dir().unwrap() {
        let f = f.unwrap();
        if f.file_type().unwrap().is_dir() {
            find_tests(&f.path(), tests);
            continue;
        }
        match f.path().extension().and_then(|s| s.to_str()) {
            Some("wat") => {}
            _ => continue,
        }
        tests.push(f.path());
    }
}

struct Test {
    file: PathBuf,
    directives: Vec<Directive>,
    assertion: Option<String>,
}

struct Directive {
    name: String,
    tys: Vec<walrus::ValType>,
}

impl Test {
    fn from_file(path: &Path) -> Result<Test> {
        let contents = fs::read_to_string(path)?;
        let mut iter = contents.lines();
        let mut assertion = None;
        let mut directives = Vec::new();
        while let Some(line) = iter.next() {
            if line.starts_with("(; CHECK-ALL:") {
                let mut pattern = String::new();
                while let Some(line) = iter.next() {
                    if line == ";)" {
                        break;
                    }
                    pattern.push_str(line);
                    pattern.push_str("\n");
                }
                while pattern.ends_with("\n") {
                    pattern.pop();
                }
                if iter.next().is_some() {
                    bail!("CHECK-ALL must be at the end of the file");
                }
                assertion = Some(pattern);
                continue;
            }

            if !line.starts_with(";; @xform") {
                continue;
            }
            let directive = &line[9..];
            let buf = wast::parser::ParseBuffer::new(directive)?;
            directives.push(wast::parser::parse::<Directive>(&buf)?);
        }
        Ok(Test {
            file: path.to_path_buf(),
            directives,
            assertion,
        })
    }

    fn check(&self, output: &str, bless: bool) -> Result<()> {
        if bless {
            update_output(&self.file, output)
        } else if let Some(pattern) = &self.assertion {
            if output == pattern {
                return Ok(());
            }
            bail!(
                "expected\n    {}\n\nactual\n    {}",
                pattern.replace("\n", "\n    "),
                output.replace("\n", "\n    ")
            );
        } else {
            bail!(
                "no test assertions were found in this file, but you can \
                 rerun tests with `BLESS=1` to automatically add assertions \
                 to this file"
            );
        }
    }
}

fn update_output(path: &Path, output: &str) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let start = contents.find("(; CHECK-ALL:").unwrap_or(contents.len());

    let mut new_output = String::new();
    for line in output.lines() {
        new_output.push_str(line);
        new_output.push_str("\n");
    }
    let new = format!(
        "{}\n\n(; CHECK-ALL:\n{}\n;)\n",
        contents[..start].trim(),
        new_output.trim_end()
    );
    fs::write(path, new)?;
    Ok(())
}

impl<'a> Parse<'a> for Directive {
    fn parse(parser: Parser<'a>) -> wast::parser::Result<Self> {
        use wast::{kw, ValType};

        parser.parse::<kw::export>()?;
        let name = parser.parse()?;
        let mut tys = Vec::new();
        parser.parens(|p| {
            while !p.is_empty() {
                tys.push(match p.parse()? {
                    ValType::I32 => walrus::ValType::I32,
                    ValType::I64 => walrus::ValType::I64,
                    ValType::F32 => walrus::ValType::F32,
                    ValType::F64 => walrus::ValType::F64,
                    _ => panic!(),
                });
            }
            Ok(())
        })?;
        Ok(Directive { name, tys })
    }
}
