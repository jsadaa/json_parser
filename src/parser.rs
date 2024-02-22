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
        match self.tokens.peek() {
            Some(&JsonToken::LeftBrace) => {
                self.tokens.next();
                self.parse_object()
            },
            Some(&JsonToken::LeftBracket) => {
                self.tokens.next();
                self.parse_array()
            },
            _ => Err("No tokens to parse".to_string()),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        let mut arr = Vec::new();

        loop {
            self.skip_whitespace();

            match self.tokens.peek() {
                Some(&JsonToken::RightBracket) => {
                    self.tokens.next(); break;
                }
                Some(&JsonToken::Comma) => {
                    self.tokens.next();
                    self.skip_whitespace();

                    if let Some(JsonToken::RightBracket) = self.tokens.peek() {
                        return Err("Unexpected comma before right bracket".to_string());
                    }
                    continue;
                }
                _ => match self.parse_value() {
                    Ok(value) => {
                        arr.push(Box::new(value));
                    }
                    Err(err) => return Err(err),
                },
            }
            self.skip_whitespace();

            if let Some(JsonToken::Comma) = self.tokens.peek() {
                self.tokens.next();
                self.skip_whitespace();

                if let Some(JsonToken::RightBracket) = self.tokens.peek() {
                    return Err("Unexpected comma before right bracket".to_string());
                }
            }
        }
        Ok(JsonValue::JsonArray(arr))
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut obj = Vec::new();

        loop {
            self.skip_whitespace();

            match self.tokens.peek() {
                Some(JsonToken::String(key)) => {
                    self.tokens.next();
                    self.skip_whitespace();

                    match self.tokens.next() {
                        Some(JsonToken::Colon) => match self.parse_value() {
                            Ok(value) => {
                                obj.push((key.clone(), Box::new(value)));
                            }
                            Err(err) => return Err(err),
                        },
                        _ => return Err("Expected colon after key in object".to_string()),
                    }
                },
                Some(&JsonToken::RightBrace) => {
                    self.tokens.next(); break;
                }
                _ => return Err("Expected string key or right brace".to_string()),
            }

            self.skip_whitespace();
            if let Some(JsonToken::Comma) = self.tokens.peek() {
                self.tokens.next();
                self.skip_whitespace();

                if let Some(JsonToken::RightBrace) = self.tokens.peek() {
                    return Err("Unexpected comma before right brace".to_string());
                }
            }
        }
        Ok(JsonValue::JsonObject(obj))
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();

        match self.tokens.peek() {
            Some(JsonToken::LeftBrace) => {
                self.tokens.next();
                self.parse_object()
            },
            Some(JsonToken::LeftBracket) => {
                self.tokens.next();
                self.parse_array()
            },
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