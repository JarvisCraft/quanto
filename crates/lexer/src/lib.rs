//! Lexer for Quanto language.

mod error;
mod source;
mod stream;
mod token;

pub use error::*;
pub use source::*;
pub use stream::*;
pub use token::*;

pub fn lex(source: &str) -> Result<TokenStream<'_>, SyntaxError> {
    use logos::Logos;
    let tokens = Token::lexer(source)
        .spanned()
        .map(|(result, span)| match result {
            Ok(token) => Ok((token, span)),
            Err(cause) => Err(SyntaxError { cause }),
        })
        .collect::<Result<_, _>>()?;

    Ok(TokenStream {
        size: source.len(),
        tokens,
    })
}

// TODO: add source information
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[error(transparent)]
pub struct SyntaxError {
    #[from]
    cause: Error,
}
