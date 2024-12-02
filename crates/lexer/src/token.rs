//! Quanto tokens

use logos::{Lexer, Logos};
use num_bigint::BigInt;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(error = crate::Error)]
pub enum Token<'source> {
    #[regex(r"[ \t\r\n\f]+")]
    Whitespace(&'source str),

    #[regex(r"//[^\n]")]
    Comment,

    // Literals
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    BoolLit(bool),

    #[regex(r"((\d+\.?\d*)|(\.\d+))(([eE][+-]?)?\d+)?", float, priority = 1)]
    FloatLit(f64),

    // Literals
    #[regex("[0-9]+", integer, priority = 2)]
    IntegerLit(BigInt),

    // TODO: allow more quoted characters
    #[regex(r#""([^"\r\n\\]|\\["nrt0\\])*""#, string)]
    StringLit(String),

    #[token("_", priority = 1)]
    IgnoredPat,

    // User entities
    #[regex(r"[a-zA-Z_][_a-zA-Z\d]*", priority = 2)]
    Ident(&'source str),

    // Punctuation
    #[token("::")]
    Namespace,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,

    #[token("=")]
    Assign,

    // Parens
    #[token("(")]
    ParenBegin,
    #[token(")")]
    ParenEnd,

    // Brackets
    #[token("[")]
    BracketBegin,
    #[token("]")]
    BracketEnd,

    // Braces
    #[token("{")]
    BraceBegin,
    #[token("}")]
    BraceEnd,

    // Maths
    #[token("+", callback = |_| BinOp::Math(MathBinOp::Add))]
    #[token("-", callback = |_| BinOp::Math(MathBinOp::Sub))]
    #[token("*", callback = |_| BinOp::Math(MathBinOp::Mul))]
    #[token("/", callback = |_| BinOp::Math(MathBinOp::Div))]
    #[token("%", callback = |_| BinOp::Math(MathBinOp::Mod))]
    // Comparison
    #[token("==", callback = |_| BinOp::Cmp(CmpOp::Eq))]
    #[token("!=", callback = |_| BinOp::Cmp(CmpOp::Ne))]
    #[token(">", callback = |_| BinOp::Cmp(CmpOp::Gt))]
    #[token(">=", callback = |_| BinOp::Cmp(CmpOp::Ge))]
    #[token("<", callback = |_| BinOp::Cmp(CmpOp::Lt))]
    #[token("<=", callback = |_| BinOp::Cmp(CmpOp::Le))]
    BinOp(BinOp),

    // Keywords

    // Basic keywords
    #[token("fn", callback = |_| Keyword::Fn)]
    #[token("mod", callback = |_| Keyword::Mod)]
    #[token("struct", callback = |_| Keyword::Struct)]
    #[token("enum", callback = |_| Keyword::Enum)]
    #[token("let", callback = |_| Keyword::Let)]
    #[token("mut", callback = |_| Keyword::Mut)]
    // Access modifiers
    #[token("pub", callback = |_| Keyword::Pub)]
    // Control flow
    #[token("if", callback = |_| Keyword::If)]
    #[token("else", callback = |_| Keyword::Else)]
    // Loop keywords
    #[token("loop", callback = |_| Keyword::Loop)]
    #[token("for", callback = |_| Keyword::For)]
    Keyword(Keyword),

    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Math(MathBinOp),
    Cmp(CmpOp),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    // Basic keywords
    Fn,
    Mod,
    Struct,
    Enum,
    Let,
    Mut,
    // Access modifiers
    Pub,
    // Control flow
    If,
    Else,
    // Loop keywords
    Loop,
    For,
}

fn string<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<String, Error> {
    let mut string = String::new();
    let mut chars = lex.slice().chars();
    let first_char = chars.next().unwrap();
    debug_assert!(
        first_char == '"',
        "the parsed string should start with a quote but starts with {first_char:?}"
    );

    let mut escaping = false;
    let mut just_escaped = false;
    let mut end_reached = false;
    for c in chars {
        debug_assert!(
            !end_reached,
            "no characters should appear after unescaped quote but got {c}"
        );

        just_escaped = false;
        if escaping {
            string.push(match c {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '0' => '\0',
                '\\' => '\\',
                _ => unreachable!(),
            });
            escaping = false;
            just_escaped = true;
        } else {
            match c {
                '\\' => {
                    escaping = true;
                }
                '"' => {
                    end_reached = true;
                }
                other => {
                    string.push(other);
                }
            }
        }
    }
    debug_assert!(!just_escaped, "there is no quote at the end of the string");

    Ok(string)
}

fn float<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<f64, Error> {
    Ok(lex.slice().parse()?)
}

fn integer<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<BigInt, Error> {
    Ok(lex.slice().parse()?)
}
