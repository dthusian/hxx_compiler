//! HXX is a basic imperative language with S-expression syntax
//! developed by me to learn compiler implementation.
//!
//! All this module needs to do is transform HXX into IR1.

use std::rc::Rc;
use lexpr::{Parser, Value};
use lexpr::datum::Ref;
use lexpr::parse::{NilSymbol, Options, TSymbol, KeywordSyntax};
use crate::common;
use crate::common::eqf64::EqF64;
use crate::common::err::{Cerr, CerrKind};
use crate::common::span::{ParseCtx, Span, SpanPlace};
use crate::common::util::invert;
use crate::ir1::model::*;

/// Parses an HXX file into IR1.
pub fn hxx_to_ir1(filename: &str, contents: &str) -> common::Result<IR1Module> {
  let ctx = Rc::new(ParseCtx {
    filename: filename.to_owned(),
    lines: contents.lines().map(|v| v.to_owned()).collect::<Vec<_>>()
  });
  let opts = Options::new()
    .with_nil_symbol(NilSymbol::EmptyList)
    .with_t_symbol(TSymbol::Default)
    .with_keyword_syntax(KeywordSyntax::ColonPrefix);
  let mut parser = Parser::from_str_custom(&contents, opts);
  let functions = parser.datum_iter()
    .map(|v|
      parse_function(
        &ctx,
        v.map_err(|e| helper_handle_lexpr_error(&ctx, e))?
          .as_ref()
      )
    )
    .collect::<common::Result<Vec<_>>>()?;
  Ok(IR1Module {
    functions,
  })
}

fn helper_handle_lexpr_error(ctx: &Rc<ParseCtx>, e: lexpr::parse::Error) -> Cerr {
  let span = e.location().map(|v|
    SpanPlace::from_loc(
      Rc::clone(&ctx),
      v
    )
  ).unwrap_or(SpanPlace {
    ctx: Rc::clone(&ctx),
    start_line: 1,
    start_col: 0,
    end_line: 1,
    end_col: 1,
  });
  Cerr::with_span(
    e,
    span
  )
}

/// In HXX, a each top-level statement (currently only functions) gets a seperate S-expr tree.
fn parse_function(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<Span<IR1Func>> {
  let list = ctx.get_list(datum)?;
  ctx.get_assert_kw(ctx.index_list(&list, 0)?, "fn")?;
  let fndecl = ctx.get_list(ctx.index_list(&list, 1)?)?;
  Ok(ctx.span_with(IR1Func {
    name: ctx.get_id(ctx.index_list(&fndecl, 0)?)?.map(|v| v.to_owned()),
    ret_typ: ctx.get_id(ctx.index_list(&list, 2)?)?.map(|v| v.to_owned()),
    args: parse_func_args(&ctx, ctx.index_list(&list, 1)?)?,
    body: parse_stmt_list(&ctx, ctx.index_list(&list, 3)?)?,
  }, datum))
}

fn parse_func_args(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<Span<IR1FuncArgs>> {
  let list = ctx.get_list(datum)?;
  let args = list.map_res(|v|
    v.into_iter()
      .skip(1)
      .map(|v| parse_var_decl(&ctx, v))
      .collect::<common::Result<Vec<_>>>()
  )?;
  Ok(args)
}

fn parse_var_decl(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<Span<IR1VarDecl>> {
  let list = ctx.get_list(datum)?;
  Ok(ctx.span_with(IR1VarDecl {
    name: ctx.get_id(ctx.index_list(&list, 0)?)?.map(|v| v.to_owned()),
    typ: ctx.get_id(ctx.index_list(&list, 1)?)?.map(|v| v.to_owned()),
  }, datum))
}

fn parse_stmt_list(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<Span<IR1StmtList>> {
  let list = ctx.get_list(datum)?;
  list.map_res(|v|
    v.into_iter()
      .map(|v|
        parse_stmt(&ctx, v)
      )
      .collect::<common::Result<Vec<_>>>()
  )
}

fn parse_stmt(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<Span<IR1Stmt>> {
  let list = ctx.get_list(datum)?;
  let block_kw_dat = ctx.index_list(&list, 0)?;
  let stmt = if block_kw_dat.as_keyword().is_some() {
    let block_kw = block_kw_dat.as_keyword().unwrap();
    if block_kw == "let" {
      IR1Stmt::Let(parse_let_stmt(&ctx, datum)?)
    } else if block_kw == "set" {
      IR1Stmt::Set(parse_set_stmt(&ctx, datum)?)
    } else if block_kw == "if" {
      IR1Stmt::If(parse_if_stmt(&ctx, datum)?)
    } else if block_kw == "while" {
      IR1Stmt::While(parse_while_stmt(&ctx, datum)?)
    } else if block_kw == "break" {
      IR1Stmt::Break
    } else {
      return Err(Cerr::with_span_of(
        CerrKind::ExpectedThing("one of \"#:let\", \"#:set\", \"#:if\", \"#:while\", or a function call".to_owned()),
        block_kw_dat,
        Rc::clone(&ctx)
      ))
    }
  } else {
    IR1Stmt::FuncCall(parse_func_call(&ctx, datum)?)
  };
  Ok(ctx.span_with(stmt, datum))
}

fn parse_let_stmt(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<IR1LetStmt> {
  let list = ctx.get_list(datum)?;
  ctx.get_assert_kw(ctx.index_list(&list, 0)?, "let")?;
  Ok(IR1LetStmt {
    decl: parse_var_decl(&ctx, ctx.index_list(&list, 1)?)?,
    init: invert(ctx.index_list(&list, 2).ok().map(|v| parse_expr(&ctx, v)))?,
  })
}

fn parse_set_stmt(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<IR1SetStmt> {
  let list = ctx.get_list(datum)?;
  ctx.get_assert_kw(ctx.index_list(&list, 0)?, "set")?;
  Ok(IR1SetStmt {
    var: ctx.get_id(ctx.index_list(&list, 1)?)?.map(|v| v.to_owned()),
    expr: parse_expr(&ctx, ctx.index_list(&list, 2)?)?,
  })
}

fn parse_if_stmt(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<IR1IfStmt> {
  let list = ctx.get_list(datum)?;
  ctx.get_assert_kw(ctx.index_list(&list, 0)?, "if")?;
  Ok(IR1IfStmt {
    cond: parse_expr(&ctx, ctx.index_list(&list, 1)?)?,
    body1: parse_stmt_list(&ctx, ctx.index_list(&list, 2)?)?,
    body2: invert(ctx.index_list(&list, 3).ok().map(|v| parse_stmt_list(&ctx, v)))?,
  })
}

fn parse_while_stmt(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<IR1WhileStmt> {
  let list = ctx.get_list(datum)?;
  ctx.get_assert_kw(ctx.index_list(&list, 0)?, "while")?;
  Ok(IR1WhileStmt {
    cond: parse_expr(&ctx, ctx.index_list(&list, 1)?)?,
    body: parse_stmt_list(&ctx, ctx.index_list(&list, 2)?)?,
  })
}

fn parse_expr(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<Span<IR1Expr>> {
  let expr = match &*datum {
    Value::Number(num) => {
      if let Some(n) = num.as_u64() {
        IR1Expr::Const(IR1Constant::U64(n))
      } else if let Some(n) = num.as_i64() {
        IR1Expr::Const(IR1Constant::I64(n))
      } else if let Some(n) = num.as_f64() {
        IR1Expr::Const(IR1Constant::F64(EqF64(n)))
      } else {
        unreachable!()
      }
    }
    Value::Symbol(symbol) => {
      IR1Expr::VarName((**symbol).to_owned())
    }
    Value::Cons(_) => {
      IR1Expr::FuncCall(parse_func_call(&ctx, datum)?)
    }
    _ => return Err(ctx.make_cerr(CerrKind::InvalidSyntax, datum))
  };
  Ok(ctx.span_with(expr, datum))
}

fn parse_func_call(ctx: &Rc<ParseCtx>, datum: Ref) -> common::Result<IR1FuncCall> {
  let list = ctx.get_list(datum)?;
  let func_name = ctx.get_id(ctx.index_list(&list, 0)?)?;
  let args = list.map_res(|v| {
    v.into_iter()
      .skip(1)
      .map(|v| {
        parse_expr(&ctx, v)
      })
      .collect::<common::Result<Vec<_>>>()
  })?;
  Ok(IR1FuncCall {
    name: func_name.map(|v| v.to_owned()),
    args
  })
}

/// Methods here are not actually associated with the ParseCtx object,
/// but are helpers that need data held inside it.
impl ParseCtx {
  fn index_list<'a>(self: &Rc<Self>, datums: &Span<Vec<Ref<'a>>>, idx: usize) -> common::Result<Ref<'a>> {
    datums.t
      .get(idx)
      .copied()
      .ok_or_else(|| Cerr::with_span(CerrKind::UnexpectedEndOfList, datums.span.clone().mark_end()))
  }

  fn get_list(self: &Rc<Self>, datum: Ref) -> common::Result<Span<Vec<Ref>>> {
    Ok(self.get_var(datum, |v| v.list_iter(), "list")?.map(|v| v.collect::<Vec<_>>()))
  }

  fn get_id(self: &Rc<Self>, datum: Ref) -> common::Result<Span<&str>> {
    self.get_var(datum, |v| v.value().as_symbol(), "identifier")
  }

  fn get_kw(self: &Rc<Self>, datum: Ref) -> common::Result<Span<&str>> {
    self.get_var(datum, |v| v.value().as_keyword(), "keyword")
  }

  fn get_assert_kw(self: &Rc<Self>, datum: Ref, expect: &str) -> common::Result<()> {
    let expected_str = "\"#:".to_owned() + expect + "\"";
    if self.get_var(datum, |v| v.value().as_keyword(), &expected_str)?.t != expect {
      return Err(self.make_cerr(CerrKind::ExpectedThing(expected_str), datum));
    }
    Ok(())
  }

  fn get_var<'a, T: 'a, F: FnOnce(Ref<'a>) -> Option<T>>(self: &Rc<Self>, datum: Ref<'a>, f: F, expected: &str) -> common::Result<Span<T>> {
    Ok(Span {
      span: SpanPlace::from_lexpr(Rc::clone(&self), datum.span()),
      t: f(datum)
        .ok_or_else(|| Cerr::with_span_of(CerrKind::ExpectedThing(expected.to_owned()), datum, Rc::clone(&self)))?,
    })
  }

  fn span_of(self: &Rc<Self>, datum: Ref) -> SpanPlace {
    SpanPlace::from_lexpr(Rc::clone(self), datum.span())
  }

  fn span_with<T>(self: &Rc<Self>, with: T, datum: Ref) -> Span<T> {
    Span {
      span: self.span_of(datum),
      t: with
    }
  }

  fn make_cerr<E: Into<CerrKind>>(self: &Rc<Self>, err: E, datum: Ref) -> Cerr {
    Cerr::with_span_of(err, datum, Rc::clone(&self))
  }
}