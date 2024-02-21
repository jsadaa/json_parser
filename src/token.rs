#[derive(Debug, Clone, PartialEq)]
pub enum JsonToken {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Whitespace,
    String(String),
    Colon,
    Comma,
    Number(String),
    Boolean(bool),
    Null,
}