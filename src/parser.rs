use crate::token::JsonToken;
use crate::value::JsonValue;

use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, JsonToken>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [JsonToken]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();

        match self.tokens.next() {
            Some(&JsonToken::LeftBrace) => {
                self.parse_object()
            },
            _ => Err("No tokens to parse".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut obj = Vec::new();

        loop {
            self.skip_whitespace();
            match self.tokens.next() {
                Some(JsonToken::String(key)) => {
                    self.skip_whitespace();
                    match self.tokens.next() {
                        Some(JsonToken::Colon) => match self.parse_value() {
                            Ok(value) => obj.push((key.clone(), Box::new(value))),
                            Err(err) => return Err(err),
                        },
                        _ => return Err("Expected colon after key in object".to_string()),
                    };
                    self.skip_whitespace();
                    match self.tokens.peek() {
                        Some(&JsonToken::Comma) => {
                            self.tokens.next();
                        }
                        Some(&JsonToken::RightBrace) => {
                            break;
                        }
                        _ => return Err("Expected comma or right brace".to_string()),
                    }
                }
                Some(JsonToken::RightBrace) => {
                    if obj.is_empty() {
                        break;
                    } else {
                        return Err("Unexpected right brace".to_string());
                    }
                }
                _ => return Err("Expected string key or right brace".to_string()),
            }
        }
        Ok(JsonValue::JsonObject(obj))
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();

        match self.tokens.peek() {
            Some(JsonToken::LeftBrace) => self.parse_object(),
            Some(JsonToken::String(value)) => {
                self.tokens.next();
                Ok(JsonValue::String(value.clone()))
            },
            Some(JsonToken::Number(value)) => {
                self.tokens.next();
                Ok(JsonValue::Number(value.clone().parse().unwrap()))
            },
            Some(JsonToken::Boolean(value)) => {
                self.tokens.next();
                Ok(JsonValue::Boolean(*value))
            },
            Some(JsonToken::Null) => {
                self.tokens.next();
                Ok(JsonValue::Null)
            },
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&&JsonToken::Whitespace) = self.tokens.peek() {
            self.tokens.next();
        }
    }
}