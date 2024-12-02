use std::fmt::Display;

use logos::Span;

use crate::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream<'source> {
    pub size: usize,
    pub tokens: Vec<(Token<'source>, Span)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenPosition(pub usize, pub usize);

impl Display for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(begin, end) = self;
        write!(f, "[{begin};{end}]")
    }
}

#[cfg(feature = "peg")]
mod peg_support {
    use super::*;
    use peg::{Parse, ParseElem, ParseSlice, RuleResult};

    impl Parse for TokenStream<'_> {
        type PositionRepr = TokenPosition;

        fn start(&self) -> usize {
            0
        }

        fn is_eof(&self, position: usize) -> bool {
            position >= self.tokens.len()
        }

        fn position_repr(&self, position: usize) -> Self::PositionRepr {
            match self.tokens.get(position) {
                Some((_token, span)) => TokenPosition(span.start, span.end),
                None => TokenPosition(self.size, self.size),
            }
        }
    }

    impl<'source> ParseElem<'source> for TokenStream<'source> {
        type Element = &'source Token<'source>;

        fn parse_elem(&'source self, position: usize) -> RuleResult<Self::Element> {
            match self.tokens.get(position) {
                Some((token, _)) => RuleResult::Matched(position + 1, token),
                None => RuleResult::Failed,
            }
        }
    }

    impl<'source> ParseSlice<'source> for TokenStream<'source> {
        type Slice = Vec<&'source Token<'source>>;

        fn parse_slice(&'source self, begin: usize, end: usize) -> Self::Slice {
            // TODO: decide if we can actuall use slice here instead of allocating a new array
            self.tokens[begin..end]
                .iter()
                .map(|(token, _)| token)
                .collect()
        }
    }
}
