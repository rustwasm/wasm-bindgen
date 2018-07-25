use std::cell::{RefCell, Cell};
use std::fmt;
use std::mem;

use console_error_panic_hook;
use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

/// Runtime test harness support instantiated in JS.
///
/// The node.js entry script instantiates a `Context` here which is used to
/// drive test execution.
#[wasm_bindgen]
pub struct Context {
    filter: Option<String>,
    current_test: RefCell<Option<String>>,
    succeeded: Cell<usize>,
    ignored: Cell<usize>,
    failures: RefCell<Vec<(String, String)>>,
    current_log: RefCell<String>,
    current_error: RefCell<String>,
    ignore_this_test: Cell<bool>,
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    #[doc(hidden)]
    pub fn console_log(s: &str);

    // Not using `js_sys::Error` because node's errors specifically have a
    // `stack` attribute.
    type NodeError;
    #[wasm_bindgen(method, getter, js_class = "Error", structural)]
    fn stack(this: &NodeError) -> String;

    // General-purpose conversion into a `String`.
    #[wasm_bindgen(js_name = String)]
    fn stringify(val: &JsValue) -> String;
}

#[wasm_bindgen(module = "fs", version = "*")]
extern {
    fn writeSync(fd: i32, data: &[u8]);
}

pub fn log(args: &fmt::Arguments) {
    console_log(&args.to_string());
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Context {
        console_error_panic_hook::set_once();

        Context {
            filter: None,
            current_test: RefCell::new(None),
            succeeded: Cell::new(0),
            ignored: Cell::new(0),
            failures: RefCell::new(Vec::new()),
            current_log: RefCell::new(String::new()),
            current_error: RefCell::new(String::new()),
            ignore_this_test: Cell::new(false),
        }
    }

    /// Inform this context about runtime arguments passed to the test
    /// harness.
    ///
    /// Eventually this will be used to support flags, but for now it's just
    /// used to support test filters.
    pub fn args(&mut self, args: Vec<JsValue>) {
        for arg in args {
            let arg = arg.as_string().unwrap();
            if arg.starts_with("-") {
                panic!("flag {} not supported", arg);
            } else if self.filter.is_some() {
                panic!("more than one filter argument cannot be passed");
            }
            self.filter = Some(arg);
        }
    }

    /// Executes a list of tests, returning whether any of them failed.
    ///
    /// This is the main entry point for executing tests. All the tests passed
    /// in are the JS `Function` object that was plucked off the
    /// `WebAssembly.Instance` exports list. This allows us to invoke it but
    /// still catch JS exceptions.
    pub fn run(&self, tests: Vec<JsValue>) -> bool {
        let this = JsValue::null();
        let args = Array::new();
        args.push(&JsValue::from(self as *const Context as u32));

        let noun = if tests.len() == 1 { "test" } else { "tests" };
        console_log!("running {} {}", tests.len(), noun);
        console_log!("");

        for test in tests {
            self.ignore_this_test.set(false);
            let test = Function::from(test);
            match test.apply(&this, &args) {
                Ok(_) => {
                    if self.ignore_this_test.get() {
                        self.log_ignore()
                    } else {
                        self.log_success()
                    }
                }
                Err(e) => self.log_error(e.into()),
            }
            drop(self.current_test.borrow_mut().take());
            *self.current_log.borrow_mut() = String::new();
            *self.current_error.borrow_mut() = String::new();
        }
        self.log_results();
        self.failures.borrow().len() == 0
    }

    fn log_start(&self, test: &str) {
        let mut current_test = self.current_test.borrow_mut();
        assert!(current_test.is_none());
        *current_test = Some(test.to_string());
        let data = format!("test {} ... ", test);
        writeSync(2, data.as_bytes());
    }

    fn log_success(&self) {
        writeSync(2, b"ok\n");
        self.succeeded.set(self.succeeded.get() + 1);
    }

    fn log_ignore(&self) {
        writeSync(2, b"ignored\n");
        self.ignored.set(self.ignored.get() + 1);
    }

    fn log_error(&self, err: NodeError) {
        writeSync(2, b"FAILED\n");
        let name = self.current_test.borrow().as_ref().unwrap().clone();
        let log = mem::replace(&mut *self.current_log.borrow_mut(), String::new());
        let error = mem::replace(&mut *self.current_error.borrow_mut(), String::new());
        let mut msg = String::new();
        if log.len() > 0 {
            msg.push_str("log output:\n");
            msg.push_str(&tab(&log));
            msg.push_str("\n");
        }
        if error.len() > 0 {
            msg.push_str("error output:\n");
            msg.push_str(&tab(&error));
            msg.push_str("\n");
        }
        msg.push_str("JS exception that was thrown:\n");
        msg.push_str(&tab(&err.stack()));
        self.failures.borrow_mut().push((name, msg));
    }

    fn log_results(&self) {
        let failures = self.failures.borrow();
        if failures.len() > 0 {
            console_log!("\nfailures:\n");
            for (test, logs) in failures.iter() {
                console_log!("---- {} output ----\n{}\n", test, tab(logs));
            }
            console_log!("failures:\n");
            for (test, _) in failures.iter() {
                console_log!("    {}\n", test);
            }
        } else {
            console_log!("");
        }
        console_log!(
            "test result: {}. \
             {} passed; \
             {} failed; \
             {} ignored\n",
            if failures.len() == 0 { "ok" } else { "FAILED" },
            self.succeeded.get(),
            failures.len(),
            self.ignored.get(),
        );
    }

    pub fn console_log(&self, original: &Function, args: &Array) {
        self.log(original, args, &self.current_log)
    }

    pub fn console_error(&self, original: &Function, args: &Array) {
        self.log(original, args, &self.current_error)
    }

    fn log(&self, orig: &Function, args: &Array, dst: &RefCell<String>) {
        if self.current_test.borrow().is_none() {
            drop(orig.apply(&JsValue::null(), args));
            return
        }
        let mut log = dst.borrow_mut();
        args.for_each(&mut |val, idx, _array| {
            if idx != 0 {
                log.push_str(" ");
            }
            log.push_str(&stringify(&val));
        });
        log.push_str("\n");
    }
}

impl Context {
    pub fn execute(&self, name: &str, f: impl FnOnce()) {
        self.log_start(name);
        if let Some(filter) = &self.filter {
            if !name.contains(filter) {
                self.ignore_this_test.set(true);
                return
            }
        }
        f();
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
