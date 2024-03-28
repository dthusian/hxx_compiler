use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ParseCtx {
  pub filename: String,
  pub lines: Vec<String>,
}

impl Debug for ParseCtx {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "ParseCtx({:?})", &self.filename)
  }
}

/// Represents a section of a source file.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SpanPlace {
  pub ctx: Rc<ParseCtx>,
  pub start_line: usize,
  pub start_col: usize,
  pub end_line: usize,
  pub end_col: usize,
}

impl SpanPlace {
  pub fn from_lexpr(ctx: Rc<ParseCtx>, value: lexpr::datum::Span) -> Self {
    SpanPlace {
      ctx,
      start_line: value.start().line(),
      start_col: value.start().column(),
      end_line: value.end().line(),
      end_col: value.end().column(),
    }
  }

  pub fn from_loc(ctx: Rc<ParseCtx>, value: lexpr::parse::error::Location) -> Self {
    SpanPlace {
      ctx,
      start_line: value.line(),
      start_col: value.column(),
      end_line: value.line(),
      end_col: value.column() + 1,
    }
  }

  pub fn mark_end(self) -> Self {
    SpanPlace {
      ctx: self.ctx,
      start_line: self.end_line,
      start_col: self.end_col + 1,
      end_line: self.end_line,
      end_col: self.end_col,
    }
  }
}

impl Display for SpanPlace {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.ctx.filename, self.start_line, self.start_col)
  }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Span<T> {
  pub span: SpanPlace,
  pub t: T
}

impl<T> Span<T> {
  pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Span<U> {
    Span {
      span: self.span,
      t: f(self.t),
    }
  }

  pub fn map_res<U, E, F: FnOnce(T) -> Result<U, E>>(self, f: F) -> Result<Span<U>, E> {
    Ok(Span {
      span: self.span,
      t: f(self.t)?,
    })
  }

  pub fn as_ref(&self) -> Span<&T> {
    Span {
      span: self.span.clone(),
      t: &self.t
    }
  }
}