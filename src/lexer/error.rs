use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexingError {

    #[error("Source file too large (maximum {max:?}, found {found:?})")]
    SourceTooBig {
        max:   u64,
        found: usize,
    },

    // All other kinds of `std::io::Error`
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}