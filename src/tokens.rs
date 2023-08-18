#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: Location
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    OParen,
    CParen,

    Number,
    String,

    Comma,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub file_name: String,
    pub column: usize,
    pub line_number: usize,
    pub line: String
}