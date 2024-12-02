use std::num::ParseFloatError;

use num_bigint::ParseBigIntError;

#[derive(thiserror::Error, Default, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid floating point literal: {0}")]
    InvalidFloat(#[from] ParseFloatError),
    #[error("invalid integral literal: {0}")]
    InvalidInteger(#[from] ParseBigIntError),
    #[error("invalid string literal")]
    InvalidString,
    #[default]
    #[error("Invalid syntax")]
    Invalid,
}
