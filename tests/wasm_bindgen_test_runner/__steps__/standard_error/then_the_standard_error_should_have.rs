use crate::__steps__::Context;
use assert_cmd::prelude::*;
use predicates::str;

pub fn then_the_standard_error_should_have(context: Context, content: &str) {
    let output = context.into_output().expect("No output was produced");

    output.assert().stderr(str::contains(content));
}
