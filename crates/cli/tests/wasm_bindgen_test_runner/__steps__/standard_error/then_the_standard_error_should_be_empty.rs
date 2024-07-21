use crate::__steps__::OutputContext;
use assert_cmd::prelude::*;
use predicates::str;

pub fn then_the_standard_error_should_be_empty(context: &dyn OutputContext) {
    let output = context.output().expect("No output was produced");

    output.assert().stderr(str::is_empty());
}
