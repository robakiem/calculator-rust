use pest_derive::Parser;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
mod ast;
use crate::ast::{Op, Expr}; 

#[derive(Parser)]
#[grammar = "calculator.pest"]
pub struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(unary_minus))
    };
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
             };
             Expr::BinOp {
                 lhs: Box::new(lhs),
                 op,
                 rhs: Box::new(rhs),
              }
         })
         .map_prefix(|op, rhs| match op.as_rule() {
             Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
             _ => unreachable!(),
         })
        .parse(pairs)

}