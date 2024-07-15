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
