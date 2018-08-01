// ignore-test - not a test

#![cfg(test)]

extern crate compiletest_rs as compiletest;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut config = compiletest::Config::default();
    config.mode = "ui".parse().expect("invalid mode");
    let mut me = env::current_exe().unwrap();
    me.pop();
    config.target_rustcflags = Some(format!("-L {}", me.display()));
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config.src_base = src;

    me.pop();
    me.pop();
    config.build_base = me.join("tests/ui");
    drop(fs::remove_dir_all(&config.build_base));
    compiletest::run_tests(&config);
}
