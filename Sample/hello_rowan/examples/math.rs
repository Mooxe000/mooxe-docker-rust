//! Example that takes the input
//! 1 + 2 * 3 + 4
//! and builds the tree
//! - Marker(Root)
//!   - Marker(Operation)
//!     - Marker(Operation)
//!       - "1" Token(Number)
//!       - "+" Token(Add)
//!       - Marker(Operation)
//!         - "2" Token(Number)
//!         - "*" Token(Mul)
//!         - "3" Token(Number)
//!     - "+" Token(Add)
//!     - "4" Token(Number)

use rowan::{
  GreenNodeBuilder
, NodeOrToken
}
;

mod math_mod
;
use math_mod::syntax_kind::SyntaxKind::*
;

type SyntaxKind = math_mod::syntax_kind::SyntaxKind
;

use math_mod::parser::Parser
;

type SyntaxElement = math_mod::parser::SyntaxElement
;

fn print(
  indent: usize
, element: SyntaxElement
) {
  let kind: SyntaxKind = element.kind().into()
  ;
  print!(
    "{:indent$}", ""
  , indent = indent
  )
  ;
  match element
  {
    NodeOrToken::Node(node) => {
      println!("- {:?}", kind)
      ;
      for child in node.children_with_tokens()
      {
        print(indent + 2, child)
        ;
      }
    }

    NodeOrToken::Token(token) =>
      println!("- {:?} {:?}", token.text(), kind)
  }
}

fn main() {

  let ast = Parser {
    builder: GreenNodeBuilder::new()
  ,
    iter: vec![
      // 1 + 2 * 3 + 4
      (NUMBER, "1".into())
    , (WHITESPACE, " ".into())
    , (ADD, "+".into())
    , (WHITESPACE, " ".into())
    , (NUMBER, "2".into())
    , (WHITESPACE, " ".into())
    , (MUL, "*".into())
    , (WHITESPACE, " ".into())
    , (NUMBER, "3".into())
    , (WHITESPACE, " ".into())
    , (ADD, "+".into())
    , (WHITESPACE, " ".into())
    , (NUMBER, "4".into())
    ]
    .into_iter()
    .peekable()
  }
  .parse()
  ;

  print(0, ast.into());

}
