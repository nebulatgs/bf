use crate::parser::Parser;

mod interpreter;
mod parser;
mod tokens;

fn main() {
    println!("BF Interpreter in Rust:");
    loop {
        let program = read_line();
        let mut parser = Parser::new(program);
        let tokens = parser.into_token_stream().collect::<Vec<_>>();

        let mut interpreter = Interpreter::new(tokens);
        interpreter.run();
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
