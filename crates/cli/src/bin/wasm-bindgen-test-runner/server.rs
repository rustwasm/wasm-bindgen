use std::ffi::OsString;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;

use failure::{Error, ResultExt};
use rouille::{self, Request, Response, Server};
use wasm_bindgen_cli_support::wasm2es6js::Config;

pub fn spawn(
    addr: &SocketAddr,
    headless: bool,
    module: &str,
    tmpdir: &Path,
    args: &[OsString],
    tests: &[String],
) -> Result<Server<impl Fn(&Request) -> Response + Send + Sync>, Error> {
    let mut js_to_execute = format!(
        r#"
        import {{
            Context,
            __wbgtest_console_debug,
            __wbgtest_console_log,
            __wbgtest_console_info,
            __wbgtest_console_warn,
            __wbgtest_console_error
        }} from './{0}';
        import * as wasm from './{0}_bg';

        // Now that we've gotten to the point where JS is executing, update our
        // status text as at this point we should be asynchronously fetching the
        // wasm module.
        document.getElementById('output').textContent = "Loading wasm module...";

        async function main(test) {{
            // this is a facet of using wasm2es6js, a hack until browsers have
            // native ESM support for wasm modules.
            await wasm.booted;

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
    );
    for test in tests {
        js_to_execute.push_str(&format!("tests.push('{}');\n", test));
    }
    js_to_execute.push_str("main(tests);\n");

    let js_path = tmpdir.join("run.js");
    fs::write(&js_path, js_to_execute).context("failed to write JS file")?;

    // No browser today supports a wasm file as ES modules natively, so we need
    // to shim it. Use `wasm2es6js` here to fetch an appropriate URL and look
    // like an ES module with the wasm module under the hood.
    //
    // TODO: don't reparse the wasm module here, should pass the
    //       `parity_wasm::Module struct` directly from the output of
    //       `wasm-bindgen` previously here and avoid unnecessary
    //       parsing.
    let wasm_name = format!("{}_bg.wasm", module);
    let wasm = fs::read(tmpdir.join(&wasm_name))?;
    let output = Config::new()
        .fetch(Some(format!("/{}", wasm_name)))
        .generate(&wasm)?;
    let (js, wasm) = output.js_and_wasm()?;
    let wasm = wasm.unwrap();
    fs::write(tmpdir.join(format!("{}_bg.js", module)), js).context("failed to write JS file")?;
    fs::write(tmpdir.join(format!("{}_bg.wasm", module)), wasm)
        .context("failed to write wasm file")?;

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
            return Response::from_data("text/html", s);
        }

        // Otherwise we need to find the asset here. It may either be in our
        // temporary directory (generated files) or in the main directory
        // (relative import paths to JS). Try to find both locations.
        let mut response = try_asset(&request, &tmpdir);
        if !response.is_success() {
            response = try_asset(&request, ".".as_ref());
        }
        // Make sure browsers don't cache anything (Chrome appeared to with this
        // header?)
        response.headers.retain(|(k, _)| k != "Cache-Control");
        return response;
    })
    .map_err(|e| format_err!("{}", e))?;
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
            if !part.contains(".") {
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
