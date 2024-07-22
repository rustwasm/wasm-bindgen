use crate::__steps__::Context;
use assert_cmd::prelude::*;

pub fn then_failure_should_have_been_returned(context: &Context) {
    let output = context.output().expect("No output was produced");

    output.assert().failure();
}
