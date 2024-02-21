use crate::token::JsonToken;

pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, index: 0 }
    }

    pub(crate) fn tokenize(&mut self) -> Result<Vec<JsonToken>, String> {
        let mut tokens = Vec::new();
        while self.has_next() {
            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.input[self.index..].chars().next();
        if ch.is_some() {
            self.index += 1;
        }
        ch
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.index..].chars().next()
    }

    fn has_next(&self) -> bool {
        self.index < self.input.len()
    }

    fn next_token(&mut self) -> Result<JsonToken, String> {
        let ch = self.next_char();
        match ch {
            Some('{') => Ok(JsonToken::LeftBrace),
            Some('}') => Ok(JsonToken::RightBrace),
            Some('[') => Ok(JsonToken::LeftBracket),
            Some(']') => Ok(JsonToken::RightBracket),
            Some('\"') => {
                let s = self.read_string();
                Ok(JsonToken::String(s.unwrap_or_default()))
            },
            Some(',') => Ok(JsonToken::Comma),
            Some(':') => Ok(JsonToken::Colon),
            Some(' ') | Some('\t') | Some('\n') | Some('\r') => Ok(JsonToken::Whitespace),
            Some(ch) => {
                if ch.is_numeric() {
                    let num = self.read_number(ch);
                    Ok(JsonToken::Number(num))
                } else if ch == 't' || ch == 'f' || ch == 'n' {
                    match self.read_constant(ch, if ch == 'n' || ch == 't' { 3 } else { 4 }) {
                        Some(s) => match &s[..] {
                            "true" => Ok(JsonToken::Boolean(true)),
                            "false" => Ok(JsonToken::Boolean(false)),
                            "null" => Ok(JsonToken::Null),
                            _ => Err(format!("Invalid constant: {}", s)),
                        },
                        None => Err("Unexpected end of input".to_string()),
                    }
                } else {
                    Err(format!("Invalid character: {}", ch))
                }
            }
            None => Err("Unexpected end of input".to_string())
        }
    }

    fn read_constant(&mut self, initial_char: char, len: usize) -> Option<String> {
        let mut s = String::new();
        s.push(initial_char);

        for _ in 0..len {
            match self.next_char() {
                Some(ch) => s.push(ch),
                None => return None,
            }
        }

        Some(s)
    }

    fn read_string(&mut self) -> Option<String> {
        let mut s = String::new();
        loop {
            match self.next_char() {
                Some(ch) if ch == '\"' => break,
                Some(ch) => s.push(ch),
                None => return None,
            }
        }
        Some(s)
    }

    fn read_number(&mut self, initial_char: char) -> String {
        let mut num = String::new();
        num.push(initial_char);

        loop {
            match self.peek_char() {
                Some(next_ch) if next_ch.is_numeric() || next_ch == '.' => {
                    num.push(self.next_char().unwrap());
                }
                _ => break,
            }
        }

        num
    }
}