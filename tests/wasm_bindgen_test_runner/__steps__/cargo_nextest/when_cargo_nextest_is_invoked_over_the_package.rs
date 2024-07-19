use super::Context;
use crate::__steps__::cargo_nextest::cargo_nextest_command;
use crate::__steps__::TestMode;

pub fn when_cargo_nextest_is_invoked_over_the_package(context: &mut Context) {
    let mut command = cargo_nextest_command(TestMode::Default);

    command.current_dir(context.path());

    println!("Running command: {:?}", context.path());

    context.output_set(command.output());
}
