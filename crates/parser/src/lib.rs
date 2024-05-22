//! Parser for Quanto language.

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QuantoParser;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] Box<pest::error::Error<Rule>>);

impl From<pest::error::Error<Rule>> for Error {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Self::from(Box::new(value))
    }
}

pub fn validate(src: &str) -> Result<(), Error> {
    let _ = QuantoParser::parse(Rule::program, src)?;
    Ok(())
}
