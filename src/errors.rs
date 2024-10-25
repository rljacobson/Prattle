/*!

Error types.

 */

use std::fmt::{Display, Formatter};

/// These are just examples of what you might have.
pub enum EvaluationError {
  DivisionByZero,
  Underflow,
  Overflow,
  Message(String), // Useful for custom errors.
}

impl Display for EvaluationError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let msg = // the result of the following match statement
      match self {
        EvaluationError::DivisionByZero => {"Division by zero"}
        EvaluationError::Underflow => {"Underflow"}
        EvaluationError::Overflow => {"Overflow"}
        EvaluationError::Message(message) => {message.as_str()}
      };

    write!(f, "{}", msg)
  }
}



#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
