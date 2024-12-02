use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation<'source> {
    pub code: &'source str,
    pub span: Range<usize>,
}
