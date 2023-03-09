// Since these run on shell scripts, they won't work outside Unix-based OSes.
#![cfg(unix)]

use std::process::Command;
use std::str;

use example_tests::{example_dir, run, test_example};

async fn test_shell_example(name: &str, envs: &[(&str, &str)]) -> anyhow::Result<()> {
    test_example(name, || {
        let path = example_dir(name);
        run(Command::new(path.join("build.sh"))
            .current_dir(&path)
            .envs(envs.iter().copied()))?;
        Ok(path)
    })
    .await
}

macro_rules! shell_tests {
    ($(
        $(#[$attr:meta])*
        $(#[$var:literal = $val:literal])*
        $test:ident = $name:literal,
    )*) => {
        $(
            $(#[$attr])*
            #[tokio::test]
            async fn $test() -> anyhow::Result<()> {
                test_shell_example($name, &[$(($var, $val)),*]).await
            }
        )*
    };
}

shell_tests! {
    #["RUSTUP_TOOLCHAIN" = "nightly"]
    raytrace_parallel = "raytrace-parallel",
    #[ignore = "This requires module workers, which Firefox doesn't support yet."]
    synchronous_instantiation = "synchronous-instantiation",
    wasm2js = "wasm2js",
    #["RUSTUP_TOOLCHAIN" = "nightly"]
    wasm_audio_worklet = "wasm-audio-worklet",
    wasm_in_web_worker = "wasm-in-web-worker",
    websockets = "websockets",
    without_a_bundler = "without-a-bundler",
    without_a_bundler_no_modules = "without-a-bundler-no-modules",
}
