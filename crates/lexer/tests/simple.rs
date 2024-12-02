use num_bigint::BigInt;
use quanto_lexer::{BinOp, CmpOp, Keyword, MathBinOp, Token, TokenStream};

#[test]
fn simple_literal() {
    let source = "let flag = 1 + 1 == 2;";
    let parsed = quanto_lexer::lex(source).unwrap();

    assert_eq!(
        parsed,
        TokenStream {
            size: source.len(),
            tokens: vec![
                (Token::Keyword(Keyword::Let), 0..3),
                (Token::Whitespace(" "), 3..4),
                (Token::Ident("flag"), 4..8),
                (Token::Whitespace(" "), 8..9),
                (Token::Assign, 9..10),
                (Token::Whitespace(" "), 10..11),
                (Token::IntegerLit(BigInt::from(1)), 11..12),
                (Token::Whitespace(" "), 12..13),
                (Token::BinOp(BinOp::Math(MathBinOp::Add)), 13..14),
                (Token::Whitespace(" "), 14..15),
                (Token::IntegerLit(BigInt::from(1)), 15..16),
                (Token::Whitespace(" "), 16..17),
                (Token::BinOp(BinOp::Cmp(CmpOp::Eq)), 17..19),
                (Token::Whitespace(" "), 19..20),
                (Token::IntegerLit(BigInt::from(2)), 20..21),
                (Token::Semicolon, 21..22),
            ]
        }
    );
}
