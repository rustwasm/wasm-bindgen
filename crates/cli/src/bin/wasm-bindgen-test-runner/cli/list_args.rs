use clap::Args;

#[derive(Args, Debug)]
pub struct ListArgs {
    /// List all tests that would be run
    #[arg(long)]
    pub list: bool,

    /// Format of the tests listing output
    #[arg(long, value_name = "FORMAT", value_parser = ["terse", "json"])]
    pub format: Option<String>,

    /// Restricts the listing to only consider the ignored tests
    #[arg(long)]
    pub ignored: bool,
}
