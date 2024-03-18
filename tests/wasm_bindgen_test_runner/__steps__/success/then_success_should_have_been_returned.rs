use crate::__steps__::Context;
use assert_cmd::prelude::*;

pub fn then_success_should_have_been_returned(context: Context) {
    context.into_command().assert().success();
}
