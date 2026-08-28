#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Int(i64),
    Ident(String),
    Eq,
    Plus,
    Minus,
    Aesterisk,
    Slash,
    Unknown(char),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}
