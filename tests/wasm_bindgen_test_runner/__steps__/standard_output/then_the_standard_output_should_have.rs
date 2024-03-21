use crate::__steps__::Context;
use assert_cmd::prelude::*;
use predicates::str;

pub fn then_the_standard_output_should_have(context: Context, content: &str) {
    context
        .into_command()
        .assert()
        .stdout(str::contains(content));
}
