use crate::__steps__::Context;
use assert_cmd::prelude::*;
use predicates::str;

pub fn then_the_standard_error_should_have(context: Context, content: &str) {
    context
        .into_command()
        .assert()
        .stderr(str::contains(content));
}
