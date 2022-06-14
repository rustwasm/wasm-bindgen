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
    module: &str,
    tmpdir: &Path,
    args: &[OsString],
    tests: &[String],
) -> Result<Server<impl Fn(&Request) -> Response + Send + Sync>, Error> {
    let mut js_to_execute = format!(
        r#"
        import {{
            WasmBindgenTestContext as Context,
            __wbgtest_console_debug,
            __wbgtest_console_log,
            __wbgtest_console_info,
            __wbgtest_console_warn,
            __wbgtest_console_error,
            default as init,
        }} from './{0}';

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
    );
    for test in tests {
        js_to_execute.push_str(&format!("tests.push('{}');\n", test));
    }
    js_to_execute.push_str("main(tests);\n");

    let js_path = tmpdir.join("run.js");
    fs::write(&js_path, js_to_execute).context("failed to write JS file")?;

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
            return set_isolate_origin_headers(Response::from_data("text/html", s));
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
        return set_isolate_origin_headers(response);
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

    return response;
}
