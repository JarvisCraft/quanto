pub mod expr;
mod node;
pub use node::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast<M>();

#[derive(Debug, Clone, PartialEq)]
pub struct Module {}
