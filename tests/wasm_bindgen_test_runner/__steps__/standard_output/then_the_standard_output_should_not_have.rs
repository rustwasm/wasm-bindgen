use crate::__steps__::Context;
use assert_cmd::prelude::*;
use predicates::boolean::PredicateBooleanExt;
use predicates::str;

pub fn then_the_standard_output_should_not_have(context: &Context, content: &str) {
    let output = context.output().expect("No output was produced");

    output.assert().stdout(str::contains(content).not());
}
