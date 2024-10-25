/*!

A minimal abstract syntax tree (AST) implementation for a Pratt parser.

For the sake of simplicity, the AST nodes serve triple duty:

  1. Instead of having a separate `Token` type, the lexer produces AST nodes directly.
  2. The parser assembles the AST nodes into a tree structure, the standard abstract syntax tree representing the
     source text.
  3. An expression in our language is any subtree of the AST (any node with all of its descendants), including the
     whole tree itself.

Usually the expression will ultimately be represented by some tree structure. There are a variety of ways
in which the expression tree could be elaborated relative to this implementation. For example, a node
might also have a value, as in the case of a number literal, identifier, or string, or it might retain a
`Token` or `Span` instance member representing the original source text and location that resulted in the
creation of the node.

*/
#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::rc::Rc;

use itertools::join;
use crate::evaluator::Evaluator;
use crate::operator::Operator;

pub type RcASTNode<'a> = Rc<ASTNode<'a>>;
pub type Children<'c> = Vec<RcASTNode<'c>>;

/// Our primary use for an `ASTNode` is as an expression. We could have called the struct `Expression`, but I wish to
/// emphasize how building the expression tree is _syntax directed_.
///
/// ASTNodes are immutable. As a consequence, we may share subexpressions between expressions.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ASTNode<'a> {
  evaluator: Evaluator,
  children:  Children<'a>
}


impl<'a> Display for ASTNode<'a>{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

    match self {

      ASTNode::Nonterminal{
        operator,
        children
      }
        => {
        let child_list = join(&*children, ",");
        write!(f, "Nonterminal<{:?}>({})", operator.name, child_list)
      }

      ASTNode::Terminal(text) => {
        write!(f, "Terminal(\"{}\")", text)
      }

    }

  }
}


impl<'a> ASTNode<'a>{

  /// Because `ASTNode`s are immutable, `evaluate` creates a new node if it needs to.
  pub fn evaluate(&self) -> RcASTNode<'a> {
    self.evaluator.evaluate()
  }

}
