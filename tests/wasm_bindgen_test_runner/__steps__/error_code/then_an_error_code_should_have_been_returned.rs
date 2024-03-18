use crate::__steps__::Context;
use assert_cmd::prelude::*;

pub fn then_an_error_code_should_have_been_returned(context: Context) {
    context.into_command().assert().failure();
}
