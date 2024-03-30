use crate::__steps__::Context;
use assert_cmd::prelude::*;
use predicates::str;

pub fn then_the_standard_error_should_be_empty(context: Context) {
    let output = context.into_output().expect("No output was produced");

    output.assert().stderr(str::is_empty());
}
