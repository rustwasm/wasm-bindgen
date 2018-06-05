use std::process::Command;

pub(crate) fn run() {
    let output = Command::new("api-extractor")
        .arg("run")
        .output()
        .expect("api-extractor not installed?");

    let out = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };

    print!("{}", String::from_utf8_lossy(out.as_slice()));
}
