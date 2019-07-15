use lexing::Lexer;
use crate::parsing::{Parser};
use crate::evaluating::Evaluator;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::lexing::Token;

extern crate rustyline;

mod lexing;
mod parsing;
mod evaluating;

fn format_error(message: &str, token: Token) -> String {
    format!("{}:{}: {}", token.line, token.col, message)
}

fn main() {

    let mut rl = Editor::<()>::new();

    let mut evaluator = Evaluator::new();

    loop {
        let input = match rl.readline("\\>>: ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        };
        
        if input.trim_end().is_empty() { continue; }
        if input.trim_end() == "quit" { break; }
        if input.trim_end() == ":q" { break; }

        let mut lexer = Lexer::new();
        let tokens = match lexer.lex(input) {
            Ok(tokens) => tokens,
            Err(errors) => {
                println!("{:?}", errors);
                continue;
            }
        };

       println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);
        let expr = match parser.parse() {
            Ok(expr) => expr,
            Err(errors) => {
                println!("{:?}", errors);
                continue;
            }
        };

//        println!("{:#?}", expr);
        println!("Parenthesized: {}", expr);

//        evaluator.evaluate(expr);
        let redex = match evaluator.evaluate(expr) {
            Ok(expr) => expr,
            Err(errors) => {
                println!("{:?}", errors);
                continue;
            }
        };

        println!("β-reduction: {}", redex);

    }
}
