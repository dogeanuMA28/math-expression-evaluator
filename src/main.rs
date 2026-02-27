mod tokens;
mod lexer;
mod parser;
mod evaluate;

use lexer::tokenize;
use parser::is_valid_expresion;
use evaluate::evaluate;
use std::io;

fn main() {
    let mut expr = String::new();
    io::stdin().read_line(&mut expr).unwrap();

    let expr = expr.trim();

    let tokens = match tokenize(expr) {
        Ok(t) => t,
        Err(e) => {
            println!("Tokenization error: {}", e);
            return;
        }
    };

    if is_valid_expresion(tokens.clone()) {
        println!("Valid expression");
    } else {
        println!("Not a valid expression");
        return;
    }

    println!("{}", evaluate(tokens));
}
