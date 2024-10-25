/*!

Provides built in operator function definitions. A common way to do this is to have a regular function for which
the operator serves as shorthand. For example, `3 + 5` is desugered to `add(3, 5)`. With this scheme, an operator
database can refer to built-ins by name, and if a built-in operator is redefined, it's original function is still
available.

I've chosen to have distinct names for the operator and the function it desugers to. One could instead have a single
name to refer to both the function and the operator, but then _redefining_ the operator would also _rename_ the
operator, and allowing an operator to be overloaded complicates things even further. Alternatively, one could do away
with naming operators altogether and use the operator's symbols themselves whenever it needed to be referenced.
Experience has taught me that this rarely works. Sooner or later you will have a need to name them, and then all the
case that went into keeping them nameless will have been wasted.


*/

use std::intrinsics::unchecked_add;
use crate::errors::EvaluationError;
use crate::symbol_table::SymbolTable;

/// Creates entries in the symbol table for each built in function. This function obviously needs to be kept in sync
/// with whatever built-in functions exist.
pub fn register_builtins(symbol_table: &mut SymbolTable){
  symbol_table.register("add", )
}


pub fn add(a: f64, b: f64) -> Result<f64, EvaluationError>{
  match checked_add(a, b) {
    Some(value) => Ok(value),
    None => Err(EvaluationError::Overflow)
  }
}


pub fn subtract(a: f64, b: f64) -> Result<f64, EvaluationError>{
  match checked_subtract(a, b) {
    Some(value) => Ok(value),
    None => Err(EvaluationError::Overflow)
  }
}


#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
