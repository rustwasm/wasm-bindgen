//! Internal-only runtime module used for the `wasm_bindgen_test` crate.
//!
//! No API contained in this module will respect semver, these should all be
//! considered private APIs.

// # Architecture of `wasm_bindgen_test`
//
// This module can seem a bit funky, but it's intended to be the runtime support
// of the `#[wasm_bindgen_test]` macro and be amenable to executing wasm test
// suites. The general idea is that for a wasm test binary there will be a set
// of functions tagged `#[wasm_bindgen_test]`. It's the job of the runtime
// support to execute all of these functions, collecting and collating the
// results.
//
// This runtime support works in tandem with the `wasm-bindgen-test-runner`
// binary as part of the `wasm-bindgen-cli` package.
//
// ## High Level Overview
//
// Here's a rough and (semi) high level overview of what happens when this crate
// runs.
//
// * First, the user runs `cargo test --target wasm32-unknown-unknown`
//
// * Cargo then compiles all the test suites (aka `tests/*.rs`) as wasm binaries
//   (the `bin` crate type). These binaries all have entry points that are
//   `main` functions, but it's actually not used. The binaries are also
//   compiled with `--test`, which means they're linked to the standard `test`
//   crate, but this crate doesn't work on wasm and so we bypass it entirely.
//
// * Instead of using `#[test]`, which doesn't work, users wrote tests with
//   `#[wasm_bindgen_test]`. This macro expands to a bunch of `#[no_mangle]`
//   functions with known names (currently named `__wbg_test_*`).
//
// * Next up, Cargo was configured via its test runner support to execute the
//   `wasm-bindgen-test-runner` binary. Instead of what Cargo normally does,
//   executing `target/wasm32-unknown-unknown/debug/deps/foo-xxxxx.wasm` (which
//   will fail as we can't actually execute was binaries), Cargo will execute
//   `wasm-bindgen-test-runner target/.../foo-xxxxx.wasm`.
//
// * The `wasm-bindgen-test-runner` binary takes over. It runs `wasm-bindgen`
//   over the binary, generating JS bindings and such. It also figures out if
//   we're running in node.js or a browser.
//
// * The `wasm-bindgen-test-runner` binary generates a JS entry point. This
//   entry point creates a `Context` below. The runner binary also parses the
//   wasm file and finds all functions that are named `__wbg_test_*`. The
//   generate file gathers up all these functions into an array and then passes
//   them to `Context` below. Note that these functions are passed as *JS
//   values*.
//
// * Somehow, the runner then executes the JS file. This may be with node.js, it
//   may serve up files in a server and wait for the user, or it serves up files
//   in a server and starts headless testing.
//
// * Testing starts, it loads all the modules using either ES imports or Node
//   `require` statements. Everything is loaded in JS now.
//
// * A `Context` is created. The `Context` is forwarded the CLI arguments of the
//   original `wasm-bindgen-test-runner` in an environment specific fashion.
//   This is used for test filters today.
//
// * The `Context::run` function is called. Again, the generated JS has gathered
//   all wasm tests to be executed into a list, and it's passed in here.
//
// * Next, `Context::run` returns a `Promise` representing the eventual
//   execution of all the tests. The Rust `Future` that's returned will work
//   with the tests to ensure that everything's executed by the time the
//   `Promise` resolves.
//
// * When a test executes, it's executing an entry point generated by
//   `#[wasm_bindgen_test]`. The test informs the `Context` of its name and
//   other metadata, and then `Context::execute_*` function creates a future
//   representing the execution of the test. This feeds back into the future
//   returned by `Context::run` to finish the test suite.
//
// * Finally, after all tests are run, the `Context`'s future resolves, prints
//   out all the result, and finishes in JS.
//
// ## Other various notes
//
// Phew, that was a lot! Some other various bits and pieces you may want to be
// aware of are throughout the code. These include things like how printing
// results is different in node vs a browser, or how we even detect if we're in
// node or a browser.
//
// Overall this is all somewhat in flux as it's pretty new, and feedback is
// always of course welcome!


use std::cell::{RefCell, Cell};
use std::fmt;
use std::rc::Rc;

use console_error_panic_hook;
use futures::future;
use futures::prelude::*;
use js_sys::{Array, Function, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::rust2js;

// Maximum number of tests to execute concurrently.
const CONCURRENCY: usize = 8;

pub mod node;
pub mod browser;
pub mod detect;

/// Runtime test harness support instantiated in JS.
///
/// The node.js entry script instantiates a `Context` here which is used to
/// drive test execution.
#[wasm_bindgen]
pub struct Context {
    state: Rc<State>,
}

struct State {
    /// An optional filter used to restrict which tests are actually executed
    /// and which are ignored. This is passed via the `args` function which
    /// comes from the command line of `wasm-bindgen-test-runner`. Currently
    /// this is the only "CLI option"
    filter: RefCell<Option<String>>,

    /// Counter of the number of tests that have succeeded.
    succeeded: Cell<usize>,

    /// Counter of the number of tests that have been ignored
    ignored: Cell<usize>,

    /// A list of all tests which have failed.
    ///
    /// Each test listed here is paired with a `JsValue` that represents the
    /// exception thrown which caused the test to fail.
    failures: RefCell<Vec<(Test, JsValue)>>,

    /// Remaining tests to execute, when empty we're just waiting on the
    /// `Running` tests to finish.
    remaining: RefCell<Vec<Test>>,

    /// List of currently executing tests. These tests all involve some level
    /// of asynchronous work, so they're sitting on the running list.
    running: RefCell<Vec<Test>>,

    /// How to actually format output, either node.js or browser-specific
    /// implementation.
    formatter: Box<Formatter>,
}

/// Representation of one test that needs to be executed.
///
/// Tests are all represented as futures, and tests perform no work until their
/// future is polled.
struct Test {
    name: String,
    future: Box<Future<Item = (), Error = JsValue>>,
    output: Rc<RefCell<Output>>,
}

/// Captured output of each test.
#[derive(Default)]
struct Output {
    log: String,
    error: String,
}

trait Formatter {
    /// Writes a line of output, typically status information.
    fn writeln(&self, line: &str);

    /// Log the result of a test, either passing or failing.
    fn log_test(&self, name: &str, result: &Result<(), JsValue>);

    /// Convert a thrown value into a string, using platform-specific apis
    /// perhaps to turn the error into a string.
    fn stringify_error(&self, val: &JsValue) -> String;
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    #[doc(hidden)]
    pub fn js_console_log(s: &str);

    // General-purpose conversion into a `String`.
    #[wasm_bindgen(js_name = String)]
    fn stringify(val: &JsValue) -> String;
}

/// Internal implementation detail of the `console_log!` macro.
pub fn log(args: &fmt::Arguments) {
    js_console_log(&args.to_string());
}

#[wasm_bindgen]
impl Context {
    /// Creates a new context ready to run tests.
    ///
    /// A `Context` is the main structure through which test execution is
    /// coordinated, and this will collect output and results for all executed
    /// tests.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Context {
        console_error_panic_hook::set_once();

        let formatter = match node::Node::new() {
            Some(node) => Box::new(node) as Box<Formatter>,
            None => Box::new(browser::Browser::new()),
        };
        Context {
            state: Rc::new(State {
                filter: Default::default(),
                failures: Default::default(),
                ignored: Default::default(),
                remaining: Default::default(),
                running: Default::default(),
                succeeded: Default::default(),
                formatter,
            }),
        }
    }

    /// Inform this context about runtime arguments passed to the test
    /// harness.
    ///
    /// Eventually this will be used to support flags, but for now it's just
    /// used to support test filters.
    pub fn args(&mut self, args: Vec<JsValue>) {
        // Here we want to reject all flags like `--foo` or `-f` as we don't
        // support anything, and also we only support at most one non-flag
        // argument as a test filter.
        //
        // Everything else is rejected.
        let mut filter = self.state.filter.borrow_mut();
        for arg in args {
            let arg = arg.as_string().unwrap();
            if arg.starts_with("-") {
                panic!("flag {} not supported", arg);
            } else if filter.is_some() {
                panic!("more than one filter argument cannot be passed");
            }
            *filter = Some(arg);
        }
    }

    /// Executes a list of tests, returning a promise representing their
    /// eventual completion.
    ///
    /// This is the main entry point for executing tests. All the tests passed
    /// in are the JS `Function` object that was plucked off the
    /// `WebAssembly.Instance` exports list.
    ///
    /// The promise returned resolves to either `true` if all tests passed or
    /// `false` if at least one test failed.
    pub fn run(&self, tests: Vec<JsValue>) -> Promise {
        let noun = if tests.len() == 1 { "test" } else { "tests" };
        self.state.formatter.writeln(&format!("running {} {}", tests.len(), noun));
        self.state.formatter.writeln("");

        // Execute all our test functions through their wasm shims (unclear how
        // to pass native function pointers around here). Each test will
        // execute one of the `execute_*` tests below which will push a
        // future onto our `remaining` list, which we'll process later.
        let args = Array::new();
        args.push(&(self as *const Context as u32).into());
        for test in tests {
            match Function::from(test).apply(&JsValue::null(), &args) {
                Ok(_) => {}
                Err(e) => {
                    panic!("exception thrown while creating a test: {}",
                           self.state.formatter.stringify_error(&e));
                }
            }
        }

        // Now that we've collected all our tests we wrap everything up in a
        // future to actually do all the processing, and pass it out to JS as a
        // `Promise`.
        let future = ExecuteTests(self.state.clone()).map(JsValue::from)
            .map_err(|e| match e {});
        rust2js(future)
    }
}

scoped_thread_local!(static CURRENT_OUTPUT: RefCell<Output>);

/// Handler for `console.log` invocations.
///
/// If a test is currently running it takes the `args` array and stringifies
/// it and appends it to the current output of the test. Otherwise it passes
/// the arguments to the original `console.log` function, psased as
/// `original`.
//
// TODO: how worth is it to actually capture the output here? Due to the nature
// of futures/js we can't guarantee that all output is captured because JS code
// could just be executing in the void and we wouldn't know which test to
// attach it to. The main `test` crate in the rust repo also has issues about
// how not all output is captured, causing some inconsistencies sometimes.
#[wasm_bindgen]
pub fn __wbgtest_console_log(original: &Function, args: &Array) {
    record(original, args, |output| &mut output.log)
}

/// Handler for `console.error` invocations.
///
/// Works the same as `console_log` above.
#[wasm_bindgen]
pub fn __wbgtest_console_error(original: &Function, args: &Array) {
    record(original, args, |output| &mut output.error)
}

fn record(orig: &Function, args: &Array, dst: impl FnOnce(&mut Output) -> &mut String) {
    if !CURRENT_OUTPUT.is_set() {
        drop(orig.apply(&JsValue::null(), args));
        return
    }

    CURRENT_OUTPUT.with(|output| {
        let mut out = output.borrow_mut();
        let dst = dst(&mut out);
        args.for_each(&mut |val, idx, _array| {
            if idx != 0 {
                dst.push_str(" ");
            }
            dst.push_str(&stringify(&val));
        });
        dst.push_str("\n");
    });
}

impl Context {
    /// Entry point for a synchronous test in wasm. The `#[wasm_bindgen_test]`
    /// macro generates invocations of this method.
    pub fn execute_sync(&self, name: &str, f: impl FnOnce() + 'static) {
        self.execute(name, future::lazy(|| Ok(f())));
    }

    /// Entry point for an asynchronous in wasm. The
    /// `#[wasm_bindgen_test(async)]` macro generates invocations of this
    /// method.
    pub fn execute_async<F>(&self, name: &str, f: impl FnOnce() -> F + 'static)
        where F: Future<Item = (), Error = JsValue> + 'static,
    {
        self.execute(name, future::lazy(f))
    }

    fn execute(
        &self,
        name: &str,
        test: impl Future<Item = (), Error = JsValue> + 'static,
    ) {
        // If our test is filtered out, record that it was filtered and move
        // on, nothing to do here.
        let filter = self.state.filter.borrow();
        if let Some(filter) = &*filter {
            if !name.contains(filter) {
                let ignored = self.state.ignored.get();
                self.state.ignored.set(ignored + 1);
                return
            }
        }

        // Looks like we've got a test that needs to be executed! Push it onto
        // the list of remaining tests.
        let output = Rc::new(RefCell::new(Output::default()));
        let future = TestFuture {
            output: output.clone(),
            test,
        };
        self.state.remaining.borrow_mut().push(Test {
            name: name.to_string(),
            future: Box::new(future),
            output,
        });
    }
}

struct ExecuteTests(Rc<State>);

enum Never {}

impl Future for ExecuteTests {
    type Item = bool;
    type Error = Never;

    fn poll(&mut self) -> Poll<bool, Never> {
        let mut running = self.0.running.borrow_mut();
        let mut remaining = self.0.remaining.borrow_mut();

        // First up, try to make progress on all active tests. Remove any
        // finished tests.
        for i in (0..running.len()).rev() {
            let result = match running[i].future.poll() {
                Ok(Async::Ready(_jsavl)) => Ok(()),
                Ok(Async::NotReady) => continue,
                Err(e) => Err(e),
            };
            let test = running.remove(i);
            self.0.log_test_result(test, result);
        }

        // Next up, try to schedule as many tests as we can. Once we get a test
        // we `poll` it once to ensure we'll receive notifications. We only
        // want to schedule up to a maximum amount of work though, so this may
        // not schedule all tests.
        while running.len() < CONCURRENCY {
            let mut test = match remaining.pop() {
                Some(test) => test,
                None => break,
            };
            let result = match test.future.poll() {
                Ok(Async::Ready(())) => Ok(()),
                Ok(Async::NotReady) => {
                    running.push(test);
                    continue
                }
                Err(e) => Err(e),
            };
            self.0.log_test_result(test, result);
        }

        // Tests are still executing, we're registered to get a notification,
        // keep going.
        if running.len() != 0 {
            return Ok(Async::NotReady)
        }

        // If there are no tests running then we must have finished everything,
        // so we shouldn't have any more remaining tests either.
        assert_eq!(remaining.len(), 0);

        self.0.print_results();
        let all_passed = self.0.failures.borrow().len() == 0;
        Ok(Async::Ready(all_passed))
    }
}

impl State {
    fn log_test_result(&self, test: Test, result: Result<(), JsValue>) {
        // Print out information about the test passing or failing
        self.formatter.log_test(&test.name, &result);

        // Save off the test for later processing when we print the final
        // results.
        match result {
            Ok(()) => self.succeeded.set(self.succeeded.get() + 1),
            Err(e) => self.failures.borrow_mut().push((test, e)),
        }
    }

    fn print_results(&self) {
        let failures = self.failures.borrow();
        if failures.len() > 0 {
            self.formatter.writeln("\nfailures:\n");
            for (test, error) in failures.iter() {
                self.print_failure(test, error);
            }
            self.formatter.writeln("failures:\n");
            for (test, _) in failures.iter() {
                self.formatter.writeln(&format!("    {}", test.name));
            }
        }
        self.formatter.writeln("");
        self.formatter.writeln(&format!(
            "test result: {}. \
             {} passed; \
             {} failed; \
             {} ignored\n",
            if failures.len() == 0 { "ok" } else { "FAILED" },
            self.succeeded.get(),
            failures.len(),
            self.ignored.get(),
        ));
    }

    fn print_failure(&self, test: &Test, error: &JsValue) {
        let mut logs = String::new();
        let output = test.output.borrow();
        if output.log.len() > 0 {
            logs.push_str("log output:\n");
            logs.push_str(&tab(&output.log));
            logs.push_str("\n");
        }
        if output.error.len() > 0 {
            logs.push_str("error output:\n");
            logs.push_str(&tab(&output.error));
            logs.push_str("\n");
        }
        logs.push_str("JS exception that was thrown:\n");
        let error_string = self.formatter.stringify_error(error);
        logs.push_str(&tab(&error_string));

        let msg = format!("---- {} output ----\n{}", test.name, tab(&logs));
        self.formatter.writeln(&msg);
    }
}

/// A wrapper future around each test
///
/// This future is what's actually executed for each test and is what's stored
/// inside of a `Test`. This wrapper future performs two critical functions:
///
/// * First, every time when polled, it configures the `CURRENT_OUTPUT` tls
///   variable to capture output for the current test. That way at least when
///   we've got Rust code running we'll be able to capture output.
///
/// * Next, this "catches panics". Right now all wasm code is configured as
///   panic=abort, but it's more like an exception in JS. It's pretty sketchy
///   to actually continue executing Rust code after an "abort", but we don't
///   have much of a choice for now.
///
///   Panics are caught here by using a shim function that is annotated with
///   `catch` so we can capture JS exceptions (which Rust panics become). This
///   way if any Rust code along the execution of a test panics we'll hopefully
///   capture it.
///
/// Note that both of the above aspects of this future are really just best
/// effort. This is all a bit of a hack right now when it comes down to it and
/// it definitely won't work in some situations. Hopefully as those situations
/// arise though we can handle them!
///
/// The good news is that everything should work flawlessly in the case where
/// tests have no output and execute successfully. And everyone always writes
/// perfect code on the first try, right? *sobs*
struct TestFuture<F> {
    output: Rc<RefCell<Output>>,
    test: F,
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(catch)]
    fn __wbg_test_invoke(f: &mut FnMut()) -> Result<(), JsValue>;
}

impl<F: Future<Error = JsValue>> Future for TestFuture<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<F::Item, F::Error> {
        let test = &mut self.test;
        let mut future_output = None;
        CURRENT_OUTPUT.set(&self.output, || {
            __wbg_test_invoke(&mut || future_output = Some(test.poll()))
        })?;
        future_output.unwrap()
    }
}

fn tab(s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        result.push_str("    ");
        result.push_str(line);
        result.push_str("\n");
    }
    return result;
}
