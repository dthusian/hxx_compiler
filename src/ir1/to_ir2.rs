use std::collections::{HashMap};
use rug::{Float, Integer};
use crate::common;
use crate::common::err::{Cerr, CerrKind};
use crate::common::sepvec::SepVec;
use crate::common::span::{Span, SpanPlace};
use crate::common::util::invert;
use crate::ir1::model::{IR1Constant, IR1Expr, IR1Func, IR1IfStmt, IR1Module, IR1Stmt, IR1StmtList, IR1VarDecl, IR1WhileStmt};
use crate::ir2::model::{IR2Expr, IR2Func, IR2FuncCall, IR2FuncDecl, IR2IfStmt, IR2IntType, IR2Program, IR2Scope, IR2SetStmt, IR2Stmt, IR2Type, IR2VarDecl, IR2WhileStmt};
use crate::ir2::type_resolve::{function_matches, infer_expr_type, ResolveType};

struct ProgramContext {
  pub functions: Vec<IR2FuncDecl>
}

struct ScopeChain<'a> {
  pub renamed_vars: Vec<IR2VarDecl>,
  pub rename_map: HashMap<String, IR2VarDecl>,
  pub next: Option<&'a ScopeChain<'a>>
}

impl<'a> ScopeChain<'a> {
  pub fn find(&self, var: &str) -> Option<&IR2VarDecl> {
    self.rename_map.get(var)
      .or(self.next.map(|v| v.find(var)).flatten())
  }
  
  pub fn define_var(&mut self, var: &str, ty: IR2Type) -> IR2VarDecl {
    // add the new variable to the rename table
    let renamed_varname = if let Some(renamed) = self.find(var) {
      // shadowing another var
      let new_renamed_var = renamed.name
        .split_once("$")
        .map(|v| format!("{}${}", v.0, v.1.parse::<u32>().unwrap() + 1))
        .unwrap_or_else(|| format!("{}$1", &renamed.name));
      // return incremented name
      new_renamed_var
    } else {
      // new var, just use the variable name
      var.to_owned()
    };
    let var_decl = IR2VarDecl {
      name: renamed_varname,
      ty,
    };
    self.renamed_vars.push(var_decl.clone());
    self.rename_map.insert(var.to_owned(), var_decl.clone());
    var_decl
  }
}

pub fn ir1_to_ir2(modules: &[IR1Module], builtins: &[IR2FuncDecl]) -> common::Result<IR2Program> {
  let mut ctx = ProgramContext {
    functions: builtins.to_owned(),
  };
  collect_toplevel_decls(modules, &mut ctx)?;
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

fn collect_toplevel_decls(modules: &[IR1Module], ctx: &mut ProgramContext) -> common::Result<()> {
  for module in modules {
    for func in &module.functions {
      ctx.functions.push(transform_func_decl(&func)?);
    }
  }
  Ok(())
}

fn transform_func(func: &Span<IR1Func>, ctx: &ProgramContext) -> common::Result<IR2Func> {
  let decl = transform_func_decl(func)?;
  let mut scope = ScopeChain {
    renamed_vars: vec![],
    rename_map: Default::default(),
    next: None,
  };
  decl.params.iter().for_each(|v| {
    scope.define_var(&v.name, v.ty.clone());
  });
  let scope = transform_stmt_list(&func.t.body, ctx, &scope)?;
  Ok(IR2Func {
    span: func.span.clone(),
    decl,
    scope,
  })
}

fn transform_func_decl(decl: &Span<IR1Func>) -> common::Result<IR2FuncDecl> {
  Ok(IR2FuncDecl {
    name: validate_identifier(&decl.t.name)?.t.clone(),
    return_ty: transform_type(&decl.t.ret_typ.as_ref().map(String::as_str))?,
    params: decl.t.args.t.iter().map(|v| transform_var_decl(&v)).collect::<common::Result<Vec<_>>>()?,
  })
}

fn transform_if(if_stmt: &IR1IfStmt, span: SpanPlace, ctx: &ProgramContext, scope: &ScopeChain) -> common::Result<IR2IfStmt> {
  let expr = transform_expr(&if_stmt.cond, ctx, scope)?;
  let scope1 = transform_stmt_list(&if_stmt.body1, ctx, scope)?;
  let scope2 = invert(if_stmt.body2.as_ref().map(|body2| transform_stmt_list(body2, ctx, scope)))?;
  Ok(IR2IfStmt {
    span,
    expr,
    scope1,
    scope2,
  })
}

fn transform_while(while_stmt: &IR1WhileStmt, span: SpanPlace, ctx: &ProgramContext, scope: &ScopeChain) -> common::Result<IR2WhileStmt> {
  let expr = transform_expr(&while_stmt.cond, ctx, scope)?;
  let scope = transform_stmt_list(&while_stmt.body, ctx, scope)?;
  Ok(IR2WhileStmt {
    span,
    expr,
    scope,
  })
}

fn transform_stmt_list(stmt_list: &Span<IR1StmtList>, ctx: &ProgramContext, scope_chain: &ScopeChain) -> common::Result<IR2Scope> {
  let mut scope = ScopeChain {
    renamed_vars: Default::default(),
    rename_map: Default::default(),
    next: Some(scope_chain),
  };
  let mut stmts = vec![];
  // find declared vars
  stmt_list.t.iter().map(|v| {
    match &v.t {
      IR1Stmt::Let(let_stmt) => {
        // transform expr and type before shadowing
        let expr = transform_expr(&let_stmt.init, &ctx, &scope)?;
        let typename = &let_stmt.decl.t.typ.as_ref().map(|v| v.as_str());
        let varname = &let_stmt.decl.t.name.t;
        let parsed_type = transform_type(typename)?;
        let var_decl = scope.define_var(varname, parsed_type);
        // emit a set expr
        stmts.push(IR2Stmt::Set(IR2SetStmt {
          span: stmt_list.span.clone(),
          var: Some(var_decl),
          value: expr,
        }));
      }
      IR1Stmt::Set(set_stmt) => {
        let expr = transform_expr(&set_stmt.expr, ctx, &scope)?;
        let var = scope.find(&set_stmt.var.t)
          .ok_or_else(|| Cerr::with_span(CerrKind::UndeclaredVariable, set_stmt.var.span.clone()))?;
        stmts.push(IR2Stmt::Set(IR2SetStmt {
          span: v.span.clone(),
          var: Some(var.clone()),
          value: expr,
        }));
      }
      IR1Stmt::If(if_stmt) => {
        stmts.push(IR2Stmt::If(transform_if(if_stmt, v.span.clone(), ctx, &scope)?));
      }
      IR1Stmt::While(while_stmt) => {
        stmts.push(IR2Stmt::While(transform_while(while_stmt, v.span.clone(), ctx, &scope)?));
      }
      IR1Stmt::FuncCall(fn_call) => {
        stmts.push(IR2Stmt::Set(IR2SetStmt {
          span: v.span.clone(),
          var: None,
          value: transform_expr(&Span {
            span: v.span.clone(),
            t: IR1Expr::FuncCall(fn_call.clone()),
          }, ctx, &scope)?,
        }));
      }
      IR1Stmt::Break => {
        stmts.push(IR2Stmt::Break(v.span.clone()));
      }
    }
    Ok(())
  })
    .collect::<common::Result<()>>()?;
  Ok(IR2Scope {
    vars: scope.renamed_vars,
    stmt_list: stmts,
  })
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
          .map(|v| infer_expr_type(v))
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
  } else if ty.t == "void" {
    Ok(IR2Type::Void)
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

fn resolve_func_call(name: &str, types: &[ResolveType], ctx: &ProgramContext, span: &SpanPlace) -> common::Result<IR2FuncDecl> {
  let matching = ctx.functions.iter()
    .filter(|v| {
      v.name == name && function_matches(v, types)
    })
    .collect::<Vec<_>>();
  if matching.len() == 0 {
    return Err(Cerr::with_span(CerrKind::NoMatchingFuncDecl, span.clone()))
  }
  if matching.len() >= 2 {
    return Err(Cerr::with_span(
      CerrKind::MultipleMatchingFuncDecls(
        SepVec(
          matching.into_iter()
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