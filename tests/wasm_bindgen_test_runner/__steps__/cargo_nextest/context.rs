use crate::__steps__::package::PackageBuilder;
use crate::__steps__::OutputContext;
use std::path::PathBuf;
use std::process::Output;

pub struct Context {
    output: Option<Result<Output, std::io::Error>>,
    package_builder: Option<PackageBuilder>,
    path: Option<PathBuf>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            output: None,
            package_builder: None,
            path: None,
        }
    }

    pub fn output(&self) -> Result<Output, &std::io::Error> {
        Ok(self.output.as_ref().unwrap().as_ref()?.clone())
    }

    pub fn output_set(&mut self, output: Result<Output, std::io::Error>) {
        self.output = Some(output);
    }

    pub fn package_builder_set(&mut self, package_builder: PackageBuilder) {
        self.package_builder = Some(package_builder);
    }

    pub fn path(&self) -> &PathBuf {
        self.path.as_ref().unwrap()
    }

    pub fn path_set(&mut self, path: PathBuf) {
        self.path = Some(path);
    }
}

impl OutputContext for Context {
    fn output(&self) -> Result<Output, &std::io::Error> {
        Ok(self.output.as_ref().unwrap().as_ref()?.clone())
    }
}
