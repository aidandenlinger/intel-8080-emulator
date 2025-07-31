use nom::error::Error as NomError;
use thiserror::Error;

/// Error type for the library.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(nom::Err<NomError<String>>),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<nom::Err<NomError<&[u8]>>> for Error {
    fn from(value: nom::Err<NomError<&[u8]>>) -> Self {
        // Turn the vecs into a hex string for better error parsing
        Self::Parse(value.map_input(|bits| format!("{bits:x?}")))
    }
}
