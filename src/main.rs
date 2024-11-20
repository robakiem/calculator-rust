use std::io::{self, BufRead};
use calculator::CalculatorParser;
use calculator::Rule;
use pest::Parser;
mod ast; 

use calculator::{parse_expr};

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match CalculatorParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                println!("Result: {}", expr.evaluate());
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}