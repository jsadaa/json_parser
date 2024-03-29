use std::iter::Peekable;
use std::str::Chars;
use crate::token::JsonToken;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub(crate) fn tokenize(&mut self) -> Result<Vec<JsonToken>, String> {
        let mut tokens: Vec<JsonToken> = Vec::new();

        while self.has_next() {
            let token: Result<JsonToken, _> = self.next_token();
            match token {
                Ok(t) => tokens.push(t),
                Err(e) => return Err(e),
            }
        }
        Ok(tokens)
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn has_next(&mut self) -> bool {
        self.peek_char().is_some()
    }

    fn next_token(&mut self) -> Result<JsonToken, String> {
        match self.next_char() {
            Some('{') => Ok(JsonToken::LeftBrace),
            Some('}') => Ok(JsonToken::RightBrace),
            Some('[') => Ok(JsonToken::LeftBracket),
            Some(']') => Ok(JsonToken::RightBracket),
            Some('\"') => {
                let s = self.read_string();
                match s {
                    Some(s) => Ok(JsonToken::String(s)),
                    None => Err("Unexpected end of input".to_string()),
                }
            },
            Some(',') => Ok(JsonToken::Comma),
            Some(':') => Ok(JsonToken::Colon),
            Some(' ') | Some('\t') | Some('\n') | Some('\r') => Ok(JsonToken::Whitespace),
            Some(ch) => {
                if ch.is_numeric() {
                    let num: String = self.read_number(ch);
                    Ok(JsonToken::Number(num))
                } else if ch == 't' || ch == 'f' || ch == 'n' {
                    let s: String = self.read_constant(if ch == 'n' || ch == 't' { 3 } else { 4 }, ch);
                    match s.as_str() {
                        "true" => Ok(JsonToken::Boolean(true)),
                        "false" => Ok(JsonToken::Boolean(false)),
                        "null" => Ok(JsonToken::Null),
                        _ => Err(format!("Invalid constant: {}", s)),
                    }
                } else {
                    Err(format!("Invalid character: {}", ch))
                }
            }
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn read_constant(&mut self, len: usize, initial_char: char) -> String {
        let mut s = String::new();
        s.push(initial_char);

        for _ in 0..len {
            match self.next_char() {
                Some(ch) => s.push(ch),
                None => break,
            }
        }

        s
    }

    fn read_string(&mut self) -> Option<String> {
        let mut s = String::new();
        loop {
            match self.peek_char() {
                Some(&'\"') => {
                    self.chars.next(); // Consume the "
                    return Some(s);
                },
                Some(ch) => {
                    s.push(*ch);
                    self.chars.next(); // Consume the current character
                },
                None => return None,
            }
        }
    }

    fn read_number(&mut self, initial_char: char) -> String {
        let mut num = String::new();
        num.push(initial_char);
        while let Some(&next_ch) = self.peek_char() {
            if next_ch.is_numeric() || next_ch == '.' {
                num.push(next_ch);
                self.chars.next(); // Consume the current character
            } else {
                break;
            }
        }
        num
    }
}