#[derive(Debug)]
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