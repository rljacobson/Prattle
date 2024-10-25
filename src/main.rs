mod operator;
// mod parser;
mod ast;
mod parser;
mod lexer;
mod interpreter;
mod builtins;
mod errors;
mod symbol_table;
mod evaluator;

use operator::*;
use crate::ast::{ASTNode};
use std::fs::File;
use std::io::{BufReader, BufRead};

const OPERATOR_DB_FILE: &str = "resources/operators.csv";

fn get_operator_table() -> () {
  let f = File::open(OPERATOR_DB_FILE)
          .expect(format!("Could not read from {}", OPERATOR_DB_FILE).as_str());
  let reader = BufReader::new(f);
  let mut operator_table = OperatorTable::new();

  for line in reader.lines() {
    let mut fields = line.unwrap().split(',');

    let new_op = Operator{
      precedence: fields.next().unwrap().parse::<u32>().unwrap(),
      name: fields.next().unwrap().to_string(),
      l_token: fields.next().unwrap().to_string(),
      n_token: fields.next().unwrap().to_string(),
      o_token: fields.next().unwrap().to_string(),
      associativity: Associativity::Null,
      affix: Affix::Null,
      arity: 0,
      evaluator: evaluator::BuiltIn(),
    };


    for field in line.unwrap().split(','){
      println!("{}", field);
    }
  }

}

fn main() {
  // Read in the operator database

  let op = Operator{
    precedence: 11,
    name: String::from("Multiplexponent"),
    l_token: None,
    n_token: None,
    o_token: None,
    associativity: Associativity::Full,
    affix: Affix::Infix,
    arity: 2,
    function: "multiplexponent".to_string()
  };

  let terminal_node = ASTNode::Terminal(
    "thisisaterminal"
  );

  let op_node = ASTNode::Nonterminal{
    operator: &op,
    children: vec![terminal_node]
  };


  // println!("The AST:\n\n{}", op_node);

  get_operator_table();
}
