use super::{prepare_tests, PackageBuilder};
use crate::__steps__::cargo_nextest::Context;

pub fn given_there_is_a_package_with(context: &mut Context, content: &str) {
    let mut builder = PackageBuilder::new("package");

    let path = builder
        .file("src/lib.rs", &prepare_tests(content))
        .finalize();

    context.package_builder_set(builder);

    context.path_set(path);
}
