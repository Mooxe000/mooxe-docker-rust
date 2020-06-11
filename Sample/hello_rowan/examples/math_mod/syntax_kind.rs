#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
  WHITESPACE = 0

, ADD
, SUB
, MUL
, DIV

, NUMBER
, ERROR
, OPERATION
, ROOT

}

impl From<SyntaxKind>
for rowan::SyntaxKind {

  fn from(kind: SyntaxKind)
  -> Self {
    Self(kind as u16)
  }

}
