use std::process::Output;

pub trait OutputContext {
    fn output(&self) -> Result<Output, &std::io::Error>;
}
