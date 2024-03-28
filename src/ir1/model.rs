use crate::common::eqf64::EqF64;
use crate::common::span::Span;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1Module {
  pub functions: Vec<Span<IR1Func>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1Func {
  pub name: Span<String>,
  pub ret_typ: Span<String>,
  pub args: Span<IR1FuncArgs>,
  pub body: Span<IR1StmtList>,
}

pub type IR1FuncArgs = Vec<Span<IR1VarDecl>>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1VarDecl {
  pub name: Span<String>,
  pub typ: Span<String>,
}

pub type IR1StmtList = Vec<Span<IR1Stmt>>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IR1Stmt {
  Let(IR1LetStmt),
  Set(IR1SetStmt),
  If(IR1IfStmt),
  While(IR1WhileStmt),
  FuncCall(IR1FuncCall),
  Break
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1LetStmt {
  pub decl: Span<IR1VarDecl>,
  pub init: Span<IR1Expr>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1SetStmt {
  pub var: Span<String>,
  pub expr: Span<IR1Expr>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1IfStmt {
  pub cond: Span<IR1Expr>,
  pub body1: Span<IR1StmtList>,
  pub body2: Option<Span<IR1StmtList>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1WhileStmt {
  pub cond: Span<IR1Expr>,
  pub body: Span<IR1StmtList>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IR1Expr {
  Const(IR1Constant),
  VarName(String),
  FuncCall(IR1FuncCall),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR1FuncCall {
  pub name: Span<String>,
  pub args: Span<Vec<Span<IR1Expr>>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IR1Constant {
  I64(i64),
  U64(u64),
  F64(EqF64)
}

