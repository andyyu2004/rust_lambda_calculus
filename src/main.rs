extern crate rustyline;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use lexing::Lexer;

use crate::evaluating::Evaluator;
use crate::lexing::Token;
use crate::parsing::{Parser, Expr};


mod lexing;
mod parsing;
mod evaluating;

fn main() {

    println!(":help");
    let mut rl = Editor::<()>::new();

    let mut evaluator = Evaluator::new();

    loop {
        let input = match rl.readline("\\>>: ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };

        if input.trim_end().is_empty() { continue; }
        if input.trim_end() == "quit" || input.trim_end() == ":q" { break; }
        if input.trim_end() == ":help" || input.trim_end() == ":h" { print_help_text(); continue; }
        if input.trim_end() == ":env" || input.trim_end() == ":e" { println!("{:?}", evaluator.env); continue; }

        let mut lexer = Lexer::new();
        let tokens = match lexer.lex(&input) {
            Ok(tokens) => tokens,
            Err(errors) => {
                println!("{:?}", errors);
                continue;
            }
        };

        println!("Tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        let expr = match parser.parse() {
            Ok(expr) => expr,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

//        println!("{:?}", expr);
//        println!("{:#?}", expr);
        println!("Parenthesized: {:?}", expr);
        println!("Standard: {}", expr);

//        println!("Renamed to t: {}", Evaluator::alpha_rename(&expr, &"x".to_string(), &"t".to_string()));

//        evaluator.evaluate(expr);
        let redex = match evaluator.evaluate(expr) {
            Ok(expr) => expr,
            Err(errors) => {
                println!("{:?}", errors);
                continue;
            }
        };

        println!("β-reduction (parenthesized): {:?}", redex);
        println!("β-reduction: {}", redex);

        println!();
    }
}


// Do not input bindings here, used to create default Combinators from string
fn force_evaluate(xs: &str) -> Expr {
    let tokens = Lexer::new().lex(xs).expect("Failed to force lex");
    let expr = Parser::new(tokens).parse().expect("Failed to force parse");
//    Evaluator::new().beta_reduce(expr).expect("Failed to force beta-reduce")
    expr
}


fn format_error(message: &str, token: &Token) -> String {
    format!("{}:{}: {}", token.line, token.col, message)
}

fn print_help_text() {
    println!();
    println!("Help");
    println!("Use backslash '\\' as lambda");
    println!("Allows syntactic sugar for multiple abstractions: \\xyz.x y z -> \\x.\\y.\\z.x y z");
    println!("Lambda variables currently must be a single lower case character, though this may change");
    println!("Spaces are required for application");
    println!("Application has higher precedence than abstraction, standard associativity rules apply");
    println!("Applying abstractions without parentheses is allowed");
    println!("i.e. \\x.x \\y.y -> \\x.(x (\\y.y))");
    println!("You are allowed to set bindings to lambda expressions");
    println!("Metavariables can either begin with a Uppercase letter followed by english alphanumerics or a '$' followed by any english alphanumerics");
    println!("Binding is expressed using the '=' operator and the right operand can be any lambda expression");
    println!("Examples: $false = \\xy.y, M = \\f.f f, Foo = x");
    println!("Some names are by default bound to combinators, {{ I, K, KI, B, T, M }} and boolean operators {{ NOT, AND, OR }}");
    println!("Use :e or :env for see current bindings");
}
