use std::borrow::Cow;
use std::ffi::OsString;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;

use anyhow::{anyhow, Context, Error};
use rouille::{Request, Response, Server};

pub fn spawn(
    addr: &SocketAddr,
    headless: bool,
    module: &'static str,
    tmpdir: &Path,
    args: &[OsString],
    tests: &[String],
    no_module: bool,
    worker: bool,
) -> Result<Server<impl Fn(&Request) -> Response + Send + Sync>, Error> {
    let mut js_to_execute = String::new();

    let wbg_import_script = if no_module {
        String::from(
            r#"
            let Context = wasm_bindgen.WasmBindgenTestContext;
            let __wbgtest_console_debug = wasm_bindgen.__wbgtest_console_debug;
            let __wbgtest_console_log = wasm_bindgen.__wbgtest_console_log;
            let __wbgtest_console_info = wasm_bindgen.__wbgtest_console_info;
            let __wbgtest_console_warn = wasm_bindgen.__wbgtest_console_warn;
            let __wbgtest_console_error = wasm_bindgen.__wbgtest_console_error;
            let init = wasm_bindgen;
            "#,
        )
    } else {
        format!(
            r#"
            import {{
                WasmBindgenTestContext as Context,
                __wbgtest_console_debug,
                __wbgtest_console_log,
                __wbgtest_console_info,
                __wbgtest_console_warn,
                __wbgtest_console_error,
                default as init,
            }} from './{}';
            "#,
            module,
        )
    };

    if worker {
        let mut worker_script = if no_module {
            format!(r#"importScripts("{0}.js");"#, module)
        } else {
            String::new()
        };

        worker_script.push_str(&wbg_import_script);

        worker_script.push_str(&format!(
            r#"
            const wrap = method => {{
                const on_method = `on_console_${{method}}`;
                self.console[method] = function (...args) {{
                    if (self[on_method]) {{
                        self[on_method](args);
                    }}
                    postMessage(["__wbgtest_" + method, args]);
                }};
            }};

            self.__wbg_test_invoke = f => f();
            self.__wbg_test_output = "";
            self.__wbg_test_output_writeln = function (line) {{
                self.__wbg_test_output += line + "\n";
                postMessage(["__wbgtest_output", self.__wbg_test_output]);
            }}

            wrap("debug");
            wrap("log");
            wrap("info");
            wrap("warn");
            wrap("error");

            async function run_in_worker(tests) {{
                const wasm = await init("./{0}_bg.wasm");
                const t = self;
                const cx = new Context();

                self.on_console_debug = __wbgtest_console_debug;
                self.on_console_log = __wbgtest_console_log;
                self.on_console_info = __wbgtest_console_info;
                self.on_console_warn = __wbgtest_console_warn;
                self.on_console_error = __wbgtest_console_error;

                cx.args({1:?});
                await cx.run(tests.map(s => wasm[s]));
            }}

            onmessage = function(e) {{
                let tests = e.data;
                run_in_worker(tests);
            }}
            "#,
            module, args,
        ));

        let worker_js_path = tmpdir.join("worker.js");
        fs::write(&worker_js_path, worker_script).context("failed to write JS file")?;

        js_to_execute.push_str(&format!(
            r#"
            // Now that we've gotten to the point where JS is executing, update our
            // status text as at this point we should be asynchronously fetching the
            // wasm module.
            document.getElementById('output').textContent = "Loading wasm module...";
            const worker = new Worker("worker.js", {{type: "{}"}});

            worker.addEventListener("message", function(e) {{
                // Checking the whether the message is from wasm_bindgen_test
                if(
                    e.data &&
                    Array.isArray(e.data) &&
                    e.data[0] &&
                    typeof e.data[0] == "string" &&
                    e.data[0].slice(0,10)=="__wbgtest_"
                ) {{
                    const method = e.data[0].slice(10);
                    const args = e.data.slice(1);

                    if (
                        method == "log" || method == "error" ||
                        method == "warn" || method == "info" ||
                        method == "debug"
                    ) {{
                        console[method].apply(undefined, args[0]);
                    }} else if (method == "output") {{
                        document.getElementById("output").textContent = args[0];
                    }}
                }}
            }});

            async function main(test) {{
                worker.postMessage(test)
            }}

            const tests = [];
            "#,
            if no_module { "classic" } else { "module" }
        ));
    } else {
        js_to_execute.push_str(&wbg_import_script);

        js_to_execute.push_str(&format!(
            r#"
            // Now that we've gotten to the point where JS is executing, update our
            // status text as at this point we should be asynchronously fetching the
            // wasm module.
            document.getElementById('output').textContent = "Loading wasm module...";

            async function main(test) {{
                const wasm = await init('./{0}_bg.wasm');

                const cx = new Context();
                window.on_console_debug = __wbgtest_console_debug;
                window.on_console_log = __wbgtest_console_log;
                window.on_console_info = __wbgtest_console_info;
                window.on_console_warn = __wbgtest_console_warn;
                window.on_console_error = __wbgtest_console_error;

                // Forward runtime arguments. These arguments are also arguments to the
                // `wasm-bindgen-test-runner` which forwards them to node which we
                // forward to the test harness. this is basically only used for test
                // filters for now.
                cx.args({1:?});

                await cx.run(test.map(s => wasm[s]));
            }}

            const tests = [];
            "#,
            module, args,
        ));
    }
    for test in tests {
        js_to_execute.push_str(&format!("tests.push('{}');\n", test));
    }
    js_to_execute.push_str("main(tests);\n");

    let js_path = tmpdir.join("run.js");
    fs::write(js_path, js_to_execute).context("failed to write JS file")?;

    // For now, always run forever on this port. We may update this later!
    let tmpdir = tmpdir.to_path_buf();
    let srv = Server::new(addr, move |request| {
        // The root path gets our canned `index.html`. The two templates here
        // differ slightly in the default routing of `console.log`, going to an
        // HTML element during headless testing so we can try to scrape its
        // output.
        if request.url() == "/" {
            let s = if headless {
                include_str!("index-headless.html")
            } else {
                include_str!("index.html")
            };
            let s = if no_module {
                s.replace(
                    "<!-- {IMPORT_SCRIPTS} -->",
                    &format!(
                        "<script src='{}.js'></script>\n<script src='run.js'></script>",
                        module
                    ),
                )
            } else {
                s.replace(
                    "<!-- {IMPORT_SCRIPTS} -->",
                    "<script src='run.js' type=module></script>",
                )
            };
            return set_isolate_origin_headers(Response::from_data("text/html", s));
        }

        // Otherwise we need to find the asset here. It may either be in our
        // temporary directory (generated files) or in the main directory
        // (relative import paths to JS). Try to find both locations.
        let mut response = try_asset(request, &tmpdir);
        if !response.is_success() {
            response = try_asset(request, ".".as_ref());
        }
        // Make sure browsers don't cache anything (Chrome appeared to with this
        // header?)
        response.headers.retain(|(k, _)| k != "Cache-Control");
        set_isolate_origin_headers(response)
    })
    .map_err(|e| anyhow!("{}", e))?;
    return Ok(srv);

    fn try_asset(request: &Request, dir: &Path) -> Response {
        let response = rouille::match_assets(request, dir);
        if response.is_success() {
            return response;
        }

        // When a browser is doing ES imports it's using the directives we
        // write in the code that *don't* have file extensions (aka we say `from
        // 'foo'` instead of `from 'foo.js'`. Fixup those paths here to see if a
        // `js` file exists.
        if let Some(part) = request.url().split('/').last() {
            if !part.contains('.') {
                let new_request = Request::fake_http(
                    request.method(),
                    format!("{}.js", request.url()),
                    request
                        .headers()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect(),
                    Vec::new(),
                );
                let response = rouille::match_assets(&new_request, dir);
                if response.is_success() {
                    return response;
                }
            }
        }
        response
    }
}

/*
 * Set the Cross-Origin-Opener-Policy and Cross-Origin_Embedder-Policy headers
 * on the Server response to enable worker context sharing, as described in:
 * https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy#certain_features_depend_on_cross-origin_isolation
 * https://security.googleblog.com/2018/07/mitigating-spectre-with-site-isolation.html
 */
fn set_isolate_origin_headers(mut response: Response) -> Response {
    response.headers.push((
        Cow::Borrowed("Cross-Origin-Opener-Policy"),
        Cow::Borrowed("same-origin"),
    ));
    response.headers.push((
        Cow::Borrowed("Cross-Origin-Embedder-Policy"),
        Cow::Borrowed("require-corp"),
    ));

    response
}
