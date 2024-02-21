mod lexer;
mod token;

fn main() {
    let file = std::fs::read_to_string("./tests/step2/invalid2.json");
    if file.is_err() {
        eprintln!("Error : {}", file.err().unwrap());
        std::process::exit(1);
    }

    let content = file.unwrap();
    let mut lexer = lexer::Lexer::new(&content);

    let tokens =  lexer.tokenize();
    if tokens.is_err() {
        eprintln!("Error : {}", tokens.err().unwrap());
        std::process::exit(1);
    }

    for token in tokens.unwrap() {
        println!("{:?}", token);
    }
    std::process::exit(0);
}
