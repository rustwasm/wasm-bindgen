use std::fs;
use std::io::ErrorKind;
use std::process::Command;
use std::sync::Once;
use std::{io, str};

use example_tests::{example_dir, manifest_dir, run, test_example};

async fn test_webpack_example(name: &str) -> anyhow::Result<()> {
    test_example(name, || {
        let manifest_dir = manifest_dir();
        let path = example_dir(name);

        fn allow_already_exists(e: io::Error) -> io::Result<()> {
            if e.kind() == ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(e)
            }
        }

        // All of the examples have the same dependencies, so we can just install
        // to the root `node_modules` once, since Node resolves packages from any
        // outer directories as well as the one containing the `package.json`.
        static INSTALL: Once = Once::new();
        INSTALL.call_once(|| {
            fs::copy(
                manifest_dir.join("_package.json"),
                manifest_dir.join("package.json"),
            )
            .map(|_| ())
            .or_else(allow_already_exists)
            .unwrap();

            run(Command::new("npm").arg("install").current_dir(manifest_dir)).unwrap();

            fs::remove_file(manifest_dir.join("package.json")).unwrap();
        });

        // Build the example.
        run(Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir(&path))?;

        Ok(path.join("dist"))
    })
    .await
}

#[allow(unused_macros)]
macro_rules! webpack_tests {
    ($(
        $(#[$attr:meta])*
        $test:ident = $name:literal,
    )*) => {
        $(
            $(#[$attr])*
            #[tokio::test]
            async fn $test() -> anyhow::Result<()> {
                test_webpack_example($name).await
            }
        )*
    };
}

#[cfg(feature = "stable")]
webpack_tests! {
    add = "add",
    canvas = "canvas",
    char = "char",
    closures = "closures",
    console_log = "console_log",
    dom = "dom",
    duck_typed_interfaces = "duck-typed-interfaces",
    fetch = "fetch",
    guide_supported_types_examples = "guide-supported-types-examples",
    hello_world = "hello_world",
    import_js = "import_js",
    julia_set = "julia_set",
    paint = "paint",
    performance = "performance",
    request_animation_frame = "request-animation-frame",
    todomvc = "todomvc",
    wasm_in_wasm_imports = "wasm-in-wasm-imports",
    wasm_in_wasm = "wasm-in-wasm",
    weather_report = "weather_report",
    webaudio = "webaudio",
    #[ignore = "The CI virtual machines don't have GPUs, so this doesn't work there."]
    webgl = "webgl",
    webrtc_datachannel = "webrtc_datachannel",
    #[ignore = "WebXR isn't supported in Firefox yet"]
    webxr = "webxr",
}
