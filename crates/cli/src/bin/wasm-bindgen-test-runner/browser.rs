use anyhow::Context;
use rayon::prelude::*;
use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::sync::Arc;

use crate::headless::{tab, TestResult};

pub fn execute(
    shell: crate::shell::Shell,
    module: &str,
    tmpdir: &Path,
    args: &[OsString],
    tests: Vec<String>,
) -> anyhow::Result<()> {
    let isolated = env::var("NO_ISOLATED").is_err();
    let headless = env::var("NO_HEADLESS").is_err();
    let debug = env::var("WASM_BINDGEN_DEBUG").is_ok();
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
    let shell = Arc::new(shell);

    let addr = if headless {
        "127.0.0.1:0"
    } else {
        "127.0.0.1:8000"
    };
    let spawn_server = |tests| {
        let srv = crate::server::spawn(
            &addr.parse().unwrap(),
            headless,
            &module,
            &tmpdir,
            &args,
            tests,
        )
        .context("failed to spawn server")?;
        let addr = srv.server_addr();
        Ok::<_, anyhow::Error>((srv, addr))
    };
    if headless && isolated && tests.len() > 1 {
        // Here instead of spawning a server and driver running all tests
        // we create a pair of server and driver per test and spawn them
        // parallely. For parallel execution we use rayon so it can automatically
        // take care of limiting the resources. After all tests finish, we aggregate
        // their results and present them to the user.
        println!("\nrunning {} tests", tests.len());
        let tests = tests.into_iter().map(|t| vec![t]).collect::<Vec<_>>();
        let results = tests
            .par_iter()
            .map(|test| {
                let (srv, addr) = spawn_server(test)?;
                let (handle, tx) = srv.stoppable();
                let shell = shell.clone();
                let (test_outcome, _) = rayon::join(
                    move || {
                        let res = crate::headless::run_isolated(&addr, shell, timeout, debug);
                        tx.send(()).map_err(Into::into).and(res)
                    },
                    move || handle.join(),
                );
                Ok::<_, anyhow::Error>((test[0].as_str(), test_outcome?))
            })
            .collect::<Vec<_>>();
        show_test_results(results)?;
    } else if headless {
        let (srv, addr) = spawn_server(&tests)?;
        std::thread::spawn(|| srv.run());
        crate::headless::run(&addr, shell, timeout, debug)?;
    } else {
        let (srv, addr) = spawn_server(&tests)?;

        // TODO: eventually we should provide the ability to exit at some point
        // (gracefully) here, but for now this just runs forever.
        println!(
            "Interactive browsers tests are now available at http://{}",
            addr
        );
        println!("");
        println!("Note that interactive mode is enabled because `NO_HEADLESS`");
        println!("is specified in the environment of this process. Once you're");
        println!("done with testing you'll need to kill this server with");
        println!("Ctrl-C.");
        return Ok(srv.run());
    }
    Ok(())
}

// Create a summary of ran tests counting passed, ignored and failed.
// Combine informations about all failed test cases and print it before the final
// summary.
fn show_test_results(
    results: Vec<Result<(&str, TestResult), anyhow::Error>>,
) -> Result<(), anyhow::Error> {
    let mut passed = 0;
    let mut failed = 0;
    let mut ignored = 0;
    let mut all_details = String::new();
    for result in results {
        let (test_name, test_result) = result?;
        match test_result {
            TestResult::Passed => passed += 1,
            TestResult::Ignored => ignored += 1,
            TestResult::Failed { details } => {
                failed += 1;
                all_details += &details;
                all_details.push('\n');
            }
            TestResult::Timeouted {
                output,
                errors,
                logs,
            } => {
                failed += 1;
                all_details += &format!(
                    "Failed to detect {} as having been run. It might have timed out.\n",
                    test_name
                );
                if !output.is_empty() {
                    all_details += "output div contained:\n";
                    all_details += &tab(&output);
                }
                if !logs.is_empty() {
                    all_details += "logs div contained:\n";
                    all_details += &tab(&logs);
                }
                if !errors.is_empty() {
                    all_details += "errors div contained:\n";
                    all_details += &tab(&errors);
                }
            }
        }
    }
    println!();
    if !all_details.is_empty() {
        println!("failures:\n");
        println!("{all_details}");
    }
    let result = if failed == 0 { "ok" } else { "FAILED" };
    println!("test result: {result}. {passed} passed; {failed} failed; {ignored} ignored");
    Ok(())
}
