use std::io;

use parsemath::parser::ParseError;

use crate::parsemath::{ast, parser::Parser};

mod parsemath;

fn main() {
    println!("Welcome to Arithmetic expression evaluator.");
    println!("Enter your arithmetic expression below:");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => {
                        println!("Error in evaluating expression. Please enter valid expression\n");
                    }
                };
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}
