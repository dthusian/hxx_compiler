use crate::ir1::model::{IR1Constant};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR2Func {
  pub line: usize,
  pub decl: IR2FuncDecl,
  pub body: Vec<IR2Stmt>,
  pub vars: Vec<IR2VarDecl>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IR2Stmt {
  Set(IR2SetStmt),
  If(IR2IfStmt),
  While(IR2WhileStmt),
  Break(usize)
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR2SetStmt {
  pub line: usize,
  pub var: Option<String>,
  pub value: IR2Expr,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR2IfStmt {
  pub line: usize,
  pub expr: IR2Expr,
  pub body1: Vec<IR2Stmt>,
  pub body2: Vec<IR2Stmt>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR2WhileStmt {
  pub line: usize,
  pub expr: IR2Expr,
  pub body: Vec<IR2Stmt>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IR2Expr {
  Const(IR1Constant),
  VarName(String),
  FuncCall(IR2FuncCall),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR2FuncCall {
  pub decl: IR2FuncDecl,
  pub args: Vec<IR2Expr>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR2FuncDecl {
  pub name: String,
  pub return_ty: String,
  pub params: Vec<IR2VarDecl>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IR2VarDecl {
  pub name: String,
  pub ty: IR2Type,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IR2Type {
  Int(IR2IntType),
  Ptr(Box<IR2Type>),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IR2IntType {
  I8, I16, I32, I64,
  U8, U16, U32, U64,
  Bool
}