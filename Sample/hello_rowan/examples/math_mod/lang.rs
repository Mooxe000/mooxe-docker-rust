use crate::math_mod::syntax_kind::SyntaxKind::{
  ROOT
}
;
type SyntaxKind = crate::math_mod::syntax_kind::SyntaxKind
;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}

impl rowan::Language for Lang {

  type Kind = SyntaxKind
  ;

  fn kind_from_raw(raw: rowan::SyntaxKind)
  -> Self::Kind {
    assert!(raw.0 <= ROOT as u16);
    unsafe {
      std::mem::transmute::<u16, SyntaxKind>(raw.0)
    }
  }

  fn kind_to_raw(kind: Self::Kind)
  -> rowan::SyntaxKind {
    kind.into()
  }

}
