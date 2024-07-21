use crate::__steps__::OutputContext;
use assert_cmd::prelude::*;

pub fn then_success_should_have_been_returned(context: &dyn OutputContext) {
    let output = context.output().expect("No output was produced");

    output.assert().success();
}
