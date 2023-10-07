use chacha20_poly1305_aead::DecryptError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadVersion(u8),
    BadMacAuth(DecryptError),
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<DecryptError> for Error {
    fn from(e: DecryptError) -> Self {
        Error::BadMacAuth(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadVersion(v) => write!(f, "Bad protocol version: {v}"),
            Error::BadMacAuth(ref e) => write!(f, "Bad MAC check: {e}"),
            Error::Io(ref e) => write!(f, "IO error: {e}"),
        }
    }
}
