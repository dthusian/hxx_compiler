use std::collections::HashSet;
use rug::{Float, Integer};
use crate::common;
use crate::common::err::{Cerr, CerrKind};
use crate::common::sepvec::SepVec;
use crate::common::span::{Span, SpanPlace};
use crate::ir1::model::{IR1Constant, IR1Expr, IR1Func, IR1IfStmt, IR1Module, IR1VarDecl};
use crate::ir2::model::{IR2Expr, IR2Func, IR2FuncCall, IR2FuncDecl, IR2IfStmt, IR2IntType, IR2Program, IR2Type, IR2VarDecl};

struct ProgramContext {
  pub functions: HashSet<IR2FuncDecl>
}

struct ScopeChain<'a> {
  pub vars: HashSet<IR2VarDecl>,
  pub next: Option<&'a ScopeChain<'a>>
}

impl<'a> ScopeChain<'a> {
  pub fn find(&self, var: &str) -> Option<&IR2VarDecl> {
    self.vars.iter()
      .find(|v| v.name == var)
      .or(self.next.map(|v| v.find(var)).flatten())
  }
}

pub fn ir1_to_ir2(modules: &[IR1Module]) -> common::Result<IR2Program> {
  let ctx = collect_toplevel_decls(modules)?;
  let program = IR2Program {
    funcs: modules.iter()
      .map(|v|
        v.functions.iter()
          .map(|v| transform_func(v, &ctx))
      )
      .flatten()
      .collect::<common::Result<Vec<_>>>()?,
  };
  Ok(program)
}

fn collect_toplevel_decls(modules: &[IR1Module]) -> common::Result<ProgramContext> {
  let mut ctx = ProgramContext {
    functions: HashSet::new(),
  };
  for module in modules {
    for func in &module.functions {
      ctx.functions.insert(transform_func_decl(&func)?);
    }
  }
  Ok(ctx)
}

fn transform_func(func: &Span<IR1Func>, ctx: &ProgramContext) -> common::Result<IR2Func> {
  let decl = transform_func_decl(func)?;
  //todo
  Ok(IR2Func {
    span: Span { span: func.span.clone(), t: () },
    decl,
    vars: vec![],
    body: vec![],
  })
}

fn transform_func_decl(decl: &Span<IR1Func>) -> common::Result<IR2FuncDecl> {
  Ok(IR2FuncDecl {
    name: validate_identifier(&decl.t.name)?.t.clone(),
    return_ty: transform_type(&decl.t.ret_typ.as_ref().map(String::as_str))?,
    params: decl.t.args.t.iter().map(|v| transform_var_decl(&v)).collect::<common::Result<Vec<_>>>()?,
  })
}

fn transform_if(if_stmt: &Span<IR1IfStmt>, ctx: &ProgramContext, scope: &ScopeChain) -> common::Result<IR2IfStmt> {
  let expr = transform_expr(&if_stmt.t.cond, ctx, scope)?;
  todo!()
}

fn transform_expr(expr: &Span<IR1Expr>, ctx: &ProgramContext, scope: &ScopeChain) -> common::Result<IR2Expr> {
  Ok(match &expr.t {
    IR1Expr::Const(c) => {
      let bigc = match c {
        IR1Constant::I64(x) => Integer::from(*x),
        IR1Constant::U64(x) => Integer::from(*x),
        IR1Constant::F64(x) => return Err(Cerr::with_span(CerrKind::FloatsLiteralsUnsupported, expr.span.clone())),
      };
      IR2Expr::Const(bigc)
    }
    IR1Expr::VarName(v) => {
      IR2Expr::Var(
        scope.find(v)
          .ok_or_else(|| Cerr::with_span(CerrKind::UndeclaredVariable, expr.span.clone()))?
          .clone()
      )
    }
    IR1Expr::FuncCall(f) => {
      // parse exprs
      let exprs = f.args.as_ref().t.iter()
        .map(|v| transform_expr(v, ctx, scope))
        .collect::<common::Result<Vec<_>>>()?;
      // resolve func call
      let decl = resolve_func_call(
        &f.name.t,
        &exprs.iter()
          .map(|v| match v {
            IR2Expr::Const(_) => ResolveFuncCallArgType::IntConstant,
            IR2Expr::Var(var) => ResolveFuncCallArgType::IR2Type(var.ty.clone()),
            IR2Expr::FuncCall(func) => ResolveFuncCallArgType::IR2Type(func.decl.return_ty.clone()),
          })
          .collect::<Vec<_>>(),
        ctx,
        &expr.span
      )?;
      IR2Expr::FuncCall(IR2FuncCall {
        decl,
        args: exprs,
      })
    }
  })
}

fn transform_var_decl(decl: &Span<IR1VarDecl>) -> common::Result<IR2VarDecl> {
  Ok(IR2VarDecl {
    name: validate_identifier(&decl.t.name)?.t.clone(),
    ty: transform_type(&decl.t.typ.as_ref().map(String::as_str))?,
  })
}

fn transform_type(ty: &Span<&str>) -> common::Result<IR2Type> {
  // is ptr?
  if ty.t.chars().nth(0).ok_or_else(|| Cerr::with_span(CerrKind::InvalidSyntax, ty.span.clone()))? == '*' {
    let mut iter = ty.t.chars();
    iter.nth(0);
    Ok(IR2Type::Ptr(
      Box::new(transform_type(&ty.as_ref().map(|v| iter.as_str()))?)
    ))
  } else {
    Ok(IR2Type::Int(
      IR2IntType::parse(&ty.t).ok_or_else(|| Cerr::with_span(CerrKind::UndeclaredType, ty.span.clone()))?
    ))
  }
}

fn validate_identifier(ident: &Span<String>) -> common::Result<&Span<String>> {
  fn validate_inner(s: &str) -> bool {
    s.chars().nth(0).is_some_and(|v| v.is_ascii_alphabetic()) &&
    s.chars().all(|v| v.is_ascii_alphanumeric() || v == '_')
  }

  if validate_inner(&ident.t) {
    Ok(ident)
  } else {
    Err(Cerr::with_span(CerrKind::InvalidIdent, ident.span.clone()))
  }
}

enum ResolveFuncCallArgType {
  IntConstant,
  IR2Type(IR2Type)
}

fn resolve_func_call(name: &str, types: &[ResolveFuncCallArgType], ctx: &ProgramContext, span: &SpanPlace) -> common::Result<IR2FuncDecl> {
  let matching = ctx.functions.iter()
    .filter(|v| {
      v.name == name && v.params.iter()
        .zip(types.iter())
        .all(|(a, b)|
          match b {
            ResolveFuncCallArgType::IntConstant => {
              if let IR2Type::Int(_) = a { true } else { false }
            }
            ResolveFuncCallArgType::IR2Type(ty) => {
              a.ty == *ty
            }
          }
        )
    })
    .collect::<Vec<_>>();
  if matching.len() == 0 {
    return Err(Cerr::with_span(CerrKind::NoMatchingFuncDecl, span.clone()))
  }
  if matching.len() >= 2 {
    return Err(Cerr::with_span(
      CerrKind::MultipleMatchingFuncDecls(
        SepVec(
          matching.iter()
            .map(IR2FuncDecl::to_string)
            .collect::<Vec<_>>(),
          "\n  "
        )
      ),
      span.clone()
    ))
  }
  Ok(matching[0].clone())
}