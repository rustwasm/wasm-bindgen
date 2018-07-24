//! Support for printing status information of a test suite in a browser.
//!
//! Currently this is quite simple, rendering the same as the console tests in
//! node.js. Output here is rendered in a `pre`, however.

use wasm_bindgen::prelude::*;
use js_sys::Error;

pub struct Browser {
    pre: Element,
}

#[wasm_bindgen]
extern {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method, structural)]
    fn getElementById(this: &HTMLDocument, id: &str) -> Element;

    type Element;
    #[wasm_bindgen(method, getter = innerHTML, structural)]
    fn inner_html(this: &Element) -> String;
    #[wasm_bindgen(method, setter = innerHTML, structural)]
    fn set_inner_html(this: &Element, html: &str);

    type BrowserError;
    #[wasm_bindgen(method, getter, structural)]
    fn stack(this: &BrowserError) -> JsValue;
}

impl Browser {
    pub fn new() -> Browser {
        let pre = document.getElementById("output");
        pre.set_inner_html("");
        Browser {
            pre,
        }
    }
}

impl super::Formatter for Browser {
    fn writeln(&self, line: &str) {
        let mut html = self.pre.inner_html();
        html.push_str(&line);
        html.push_str("\n");
        self.pre.set_inner_html(&html);
    }

    fn log_start(&self, name: &str) {
        let data = format!("test {} ... ", name);
        let mut html = self.pre.inner_html();
        html.push_str(&data);
        self.pre.set_inner_html(&html);
    }

    fn log_success(&self) {
        let mut html = self.pre.inner_html();
        html.push_str("ok\n");
        self.pre.set_inner_html(&html);
    }

    fn log_ignored(&self) {
        let mut html = self.pre.inner_html();
        html.push_str("ignored\n");
        self.pre.set_inner_html(&html);
    }

    fn log_failure(&self, err: JsValue) -> String {
        let mut html = self.pre.inner_html();
        html.push_str("FAIL\n");
        self.pre.set_inner_html(&html);

        // TODO: this should be a checked cast to `Error`
        let err = Error::from(err);
        let name = String::from(err.name());
        let message = String::from(err.message());
        let err = BrowserError::from(JsValue::from(err));
        let stack = err.stack();

        let mut header = format!("{}: {}", name, message);
        let stack = match stack.as_string() {
            Some(stack) => stack,
            None => return header,
        };

        // If the `stack` variable contains the name/message already, this is
        // probably a chome-like error which is already rendered well, so just
        // return this info
        if stack.contains(&header) {
            return stack
        }

        // Check for a firefox-like error where all lines have a `@` in them
        // separating the symbol and source
        if stack.lines().all(|s| s.contains("@")) {
            for line in stack.lines() {
                header.push_str("\n");
                header.push_str("  at");
                for part in line.split("@") {
                    header.push_str(" ");
                    header.push_str(part);
                }
            }
            return header
        }

        // Fallback to make sure we don't lose any info
        format!("{}\n{}", header, stack)
    }
}

