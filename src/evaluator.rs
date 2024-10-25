/*!

An evaluator is the component that knows how to compute the expression. It takes (a reference to) the vector of
children as an argument and returns the result of the evaluation, which itself is always an expression.

Our language will only two primitive types: a symbol (an identifier) and a number implemented as an `f64`. They
evaluate to themselves. Other evaluators need to be able to access the raw values of these primitive types.

*/



// pub trait  {
//
//   fn evaluate(&self, children: &Ch);
//
//   fn try_unwrap_value
// }


use crate::ast::{
  ASTNode,
  RcASTNode,
  Children
};

pub type BuiltInFn = fn(args: &Vec<ASTNode>) -> ASTNode;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Evaluator {
  /// The evaluator might need to look up another piece of code bound to a symbol (a "symbolic expression"). We do
  /// not resolve it until evaluation, as the thing it is bound to can change dynamically.
  Symbol(String),
  Number(f64),
  BuiltIns(BuiltInFn),

  // Other possibilities:
  // String(String),
  // Integer(i64), // or other numeric types
  // FFI(…) // Foreign function. Can us `BuiltIn` for this purpose, perhaps.

}

impl Evaluator{
  pub fn evaluate<'a>(&self, children: &Children) -> RcASTNode<'a> {
    match self {

      Evaluator::Symbol(String) => {
        // Look up the expression referenced by the symbol.

      }

      Evaluator::Number(_) => {

      }

      Evaluator::BuiltIns(_) => {

      }

    }
  }
}



#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
