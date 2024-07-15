use clap::Args;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// If specified, only executes the tests containing [testname] in their names
    pub testname: Option<String>,

    /// Run only the test with the exact name
    #[arg(long)]
    pub exact: bool,

    /// Include ignored tests in the test run
    #[arg(long)]
    pub include_ignored: bool,

    /// Disables the tests output capture
    #[arg(long)]
    pub nocapture: bool,

    /// Skip tests whose names match the given pattern
    #[arg(long, value_name = "PATTERN", num_args = 0..)]
    pub skip: Vec<String>,
}

impl RunArgs {
    pub fn to_args(&self) -> Vec<&str> {
        let mut args = Vec::<&str>::new();

        if let Some(testname) = &self.testname {
            args.push(testname);
        }

        if self.exact {
            args.push("--exact");
        }

        if self.include_ignored {
            args.push("--include-ignored");
        }

        if self.nocapture {
            args.push("--nocapture");
        }

        for skip in &self.skip {
            args.push("--skip");
            args.push(skip);
        }

        args
    }
}
