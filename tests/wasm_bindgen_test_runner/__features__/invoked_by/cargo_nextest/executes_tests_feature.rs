use crate::__steps__::cargo_nextest::when_cargo_nextest_is_invoked_over_the_package;
use crate::__steps__::cargo_nextest::Context;
use crate::__steps__::package::given_there_is_a_package_with_100_successful_tests;
use crate::__steps__::standard_error::then_the_standard_error_should_have;
use crate::__steps__::standard_error::then_the_standard_error_should_not_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_a_package_with_100_successful_tests();
    when_cargo_nextest_is_invoked_over_the_package();

    "Outputs its running 100 tests" {
        then_the_standard_error_should_have("Starting 100 tests across 1 binary");
    }

    "Outputs the tests execution summary" {
        then_the_standard_error_should_have("100 tests run: 100 passed, 0 skipped");
    }

    "Outputs no error" {
        then_the_standard_error_should_not_have("error:");
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
