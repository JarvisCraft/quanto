use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    Integer(BigInt),
    Float(f64),
    String(String),
}
