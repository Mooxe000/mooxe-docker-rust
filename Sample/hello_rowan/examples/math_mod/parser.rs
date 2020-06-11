use rowan::{
  GreenNodeBuilder
, SmolStr
}
;

use std::iter::Peekable
;

use crate::math_mod::syntax_kind::SyntaxKind::*
;

type SyntaxKind = crate::math_mod::syntax_kind::SyntaxKind
;
type Lang = crate::math_mod::lang::Lang
;

type SyntaxNode = rowan::SyntaxNode<Lang>
;
#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<Lang>
;
#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<
  SyntaxNode
, SyntaxToken
>
;

pub struct Parser <
  I: Iterator<
    Item = (
      SyntaxKind
    , SmolStr
    )
  >
> {
  pub builder: GreenNodeBuilder<'static>
, pub iter: Peekable<I>
}

impl<
  I: Iterator<
    Item = (
      SyntaxKind
    , SmolStr
    )
  >
>
Parser<I> {

  fn peek(&mut self)
  -> Option<SyntaxKind>
  {
    while
      self.iter
      .peek()
      .map( |&(t, _)|
        t == WHITESPACE
      )
      .unwrap_or(false)
    {
      self.bump()
      ;
    }
    self.iter.peek()
    .map(|&(t, _)| t)
  }

  fn bump(&mut self) {
    if let Some((token, string)) = self.iter.next() {
      self.builder
      .token(token.into(), string)
      ;
    }
  }

  fn parse_val(&mut self) {
    match self.peek()
    {
      Some(NUMBER) => self.bump()
    , _ => {
        self.builder.start_node(ERROR.into())
        ;
        self.bump()
        ;
        self.builder.finish_node()
        ;
      }
    }
  }

  fn handle_operation(
    &mut self
  , tokens: &[SyntaxKind]
  , next: fn(&mut Self)
  ) {
    let checkpoint = self.builder.checkpoint()
    ;
    next(self)
    ;
    while
      self.peek()
      .map(|t| tokens.contains(&t))
      .unwrap_or(false)
    {
      self.builder.start_node_at(checkpoint, OPERATION.into())
      ;
      self.bump()
      ;
      next(self)
      ;
      self.builder.finish_node()
      ;
    }
  }

  fn parse_mul(&mut self) {
    self.handle_operation(
      &[MUL, DIV]
    , Self::parse_val
    )
  }

  fn parse_add(&mut self) {
    self.handle_operation(
      &[ADD, SUB]
    , Self::parse_mul
    )
  }

  pub fn parse(mut self)
  -> SyntaxNode {
    self.builder.start_node(ROOT.into())
    ;
    self.parse_add()
    ;
    self.builder.finish_node()
    ;

    SyntaxNode::new_root(
      self.builder.finish()
    )
  }

}
