/*!

This contains the internal infrastructure needed to interpret code, including a symbol table and built-ins.

 */


use std::collections::HashMap;
use crate::ASTNode;
use crate::symbol_table::SymbolTable;


pub trait Evaluatable {
  fn evaluate() -> ASTNode;
}

pub struct RuntimeContext<'a> {
  symbols: SymbolTable<'a>,
  root: ASTNode<'a>
}



#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
