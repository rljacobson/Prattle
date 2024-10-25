/*!

The symbol table is a mapping from names to entities. Our entities are nodes in the AST, that is, expressions.

 */

use std::collections::HashMap;
use crate::ast::RcASTNode;


pub struct SymbolRecord{
  name: String,

}



pub struct SymbolTable<'a>(HashMap<String, RcASTNode<'a>>);





#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
