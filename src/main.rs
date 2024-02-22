use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::JsonToken;
use crate::value::{AstWrapper, JsonValue};

mod lexer;
mod token;
mod value;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file = std::fs::read_to_string(&args[1]);

    if file.is_err() {
        eprintln!("Error : {}", file.err().unwrap());
        std::process::exit(1);
    }

    let content: String = file.unwrap();
    let mut lexer: Lexer = Lexer::new(&content);

    let tokens: Result<Vec<JsonToken>, String> =  lexer.tokenize();
    if tokens.is_err() {
        eprintln!("Error : {}", tokens.err().unwrap());
        std::process::exit(1);
    }

    let tokens: Vec<JsonToken> = tokens.unwrap();

    let mut parser: Parser = Parser::new(tokens.as_slice());
    let result: Result<JsonValue, String> = parser.parse();

    if result.is_err() {
        eprintln!("Error : {}", result.err().unwrap());
        std::process::exit(1);
    }

    let parsed_value: JsonValue = result.unwrap();

    println!("{}", AstWrapper(&parsed_value));
    std::process::exit(0);
}
