use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use lexpr::Datum;
use lexpr::datum::Ref;
use thiserror::Error;
use crate::common::sepvec::SepVec;
use crate::common::span::{ParseCtx, SpanPlace};

#[derive(Debug)]
pub struct Cerr {
  pub span: SpanPlace,
  pub kind: CerrKind
}

impl Display for Cerr {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "error: at {}: {}", &self.span, &self.kind)?;
    writeln!(f)?;
    let start_line = self.span.ctx.lines.get(self.span.start_line - 1).cloned().unwrap_or("".to_owned());
    writeln!(f, "  {}", start_line)?;
    let squiggle_len = if self.span.start_line != self.span.end_line {
      start_line.len().saturating_sub(self.span.start_col)
    } else {
      self.span.end_col.saturating_sub(self.span.start_col)
    };
    let squiggle_len = squiggle_len.max(1);
    writeln!(f, "  {}{}", " ".repeat(self.span.start_col), "~".repeat(squiggle_len))?;
    if self.span.start_line != self.span.end_line {
      writeln!(f, "   ...")?;
    }
    writeln!(f)?;
    Ok(())
  }
}

impl Error for Cerr { }

impl Cerr {
  pub fn with_span<E: Into<CerrKind>>(err: E, span: SpanPlace) -> Self {
    Cerr {
      span,
      kind: err.into(),
    }
  }
  pub fn with_span_of<E: Into<CerrKind>, S: HasSpan>(err: E, val: S, ctx: Rc<ParseCtx>) -> Self {
    Cerr {
      span: SpanPlace::from_lexpr(ctx, val.span()),
      kind: err.into()
    }
  }
}

#[derive(Error, Debug)]
pub enum CerrKind {
  // HXX->IR1
  #[error("invalid s-expression")]
  InvalidSexpr(#[from] lexpr::parse::Error),
  #[error("invalid syntax")]
  InvalidSyntax,
  #[error("invalid syntax, expected {0}")]
  ExpectedThing(String),
  #[error("invalid syntax, unexpected end of list")]
  UnexpectedEndOfList,

  // IR1->IR2
  #[error("undeclared variable")]
  UndeclaredVariable,
  #[error("undeclared type")]
  UndeclaredType,
  #[error("invalid identifier")]
  InvalidIdent,
  #[error("float literals are unsupported")]
  FloatsLiteralsUnsupported,
  #[error("no matching function declaration")]
  NoMatchingFuncDecl,
  #[error("multiple matching function declarations:\n  {0}")]
  MultipleMatchingFuncDecls(SepVec<String>),
  #[error("unknown attribute")]
  UnknownAttribute,
}

pub type Result<T> = std::result::Result<T, Cerr>;

trait HasSpan {
  fn span(&self) -> lexpr::parse::Span;
}

impl HasSpan for Datum {
  fn span(&self) -> lexpr::datum::Span {
    self.span()
  }
}

impl<'a> HasSpan for Ref<'a> {
  fn span(&self) -> lexpr::datum::Span {
    self.span()
  }
}