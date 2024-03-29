use crate::__steps__::Context;
use assert_cmd::prelude::*;
use predicates::str;

pub fn then_the_standard_output_should_have(context: Context, content: &str) {
    let output = context.into_output().expect("No output was produced");

    output.assert().stdout(str::contains(content));
}
