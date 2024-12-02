use crate::ast::{expr::*, Node};
use quanto_lexer::{SourceLocation as Src, Token, TokenStream};

peg::parser! { pub grammar root<'source, 'tokens>() for TokenStream<'source> {

    rule expression() -> Node<Src<'source>, Src<'source>> = precedence!{

    }

    rule literal() -> Node<Literal, Src<'source>> =
        l:position!()
        literal:(
            ([Token::BoolLit(value)] { Literal::Bool(*value) })
            / ([Token::IntegerLit(value)] { Literal::Integer(value.clone()) })
            / ([Token::FloatLit(value)] { Literal::Float(*value) })
            / ([Token::StringLit(value)] { Literal::String(value.clone()) })
        )
        r:position!()
        { Node::new(literal, todo!()) }
}}
