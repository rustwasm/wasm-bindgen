// Since these run on shell scripts, they won't work outside Unix-based OSes.
#![cfg(unix)]

use std::process::Command;
use std::str;

use example_tests::{example_dir, run, test_example};

async fn test_shell_example(name: &str) -> anyhow::Result<()> {
    test_example(name, || {
        let path = example_dir(name);
        run(Command::new(path.join("build.sh")).current_dir(&path))?;
        Ok(path)
    })
    .await
}

macro_rules! shell_tests {
    ($(
        $(#[$attr:meta])*
        $test:ident = $name:literal,
    )*) => {
        $(
            $(#[$attr])*
            #[tokio::test]
            async fn $test() -> anyhow::Result<()> {
                test_shell_example($name).await
            }
        )*
    };
}

shell_tests! {
    #[ignore = "This requires module workers, which Firefox doesn't support yet."]
    synchronous_instantiation = "synchronous-instantiation",
    wasm2js = "wasm2js",
    wasm_in_web_worker = "wasm-in-web-worker",
    websockets = "websockets",
    without_a_bundler = "without-a-bundler",
    without_a_bundler_no_modules = "without-a-bundler-no-modules",
}

#[tokio::test]
async fn raytrace_parallel() -> anyhow::Result<()> {
    test_example("raytrace-parallel", || {
        let path = example_dir("raytrace-parallel");

        run(Command::new(path.join("build.sh"))
            .current_dir(&path)
            // This example requires nightly.
            .env("RUSTUP_TOOLCHAIN", "nightly"))?;

        Ok(path)
    })
    .await
}
