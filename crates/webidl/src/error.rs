use failure::{Backtrace, Context, Fail};
use std::fmt;

/// Either `Ok(t)` or `Err(Error)`.
pub type Result<T> = ::std::result::Result<T, Error>;

/// The different contexts an error can occur in in this crate.
#[derive(Debug, Fail, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorKind {
    /// Failed to open a WebIDL file.
    #[fail(display = "opening WebIDL file")]
    OpeningWebIDLFile,
    /// Failed to read a WebIDL file.
    #[fail(display = "reading WebIDL file")]
    ReadingWebIDLFile,
    /// Failed to parse a WebIDL file.
    #[fail(display = "parsing WebIDL source text at {}", _0)]
    ParsingWebIDLSourcePos(usize),
    /// Failed to parse a WebIDL file.
    #[fail(display = "parsing WebIDL source text")]
    ParsingWebIDLSource,
}

/// The error type for this crate.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    /// The context for this error.
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}
