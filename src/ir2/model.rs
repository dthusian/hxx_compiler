use std::fmt::{Display, Formatter};
use rug::{Integer};
use crate::common::err::{Cerr, CerrKind};
use crate::common::span::Span;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2Program {
  pub funcs: Vec<IR2Func>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2Func {
  pub span: Span<()>,
  pub decl: IR2FuncDecl,
  pub vars: Vec<IR2VarDecl>,
  pub body: Vec<IR2Stmt>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2Stmt {
  Set(IR2SetStmt),
  If(IR2IfStmt),
  While(IR2WhileStmt),
  Break(Span<()>)
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2SetStmt {
  pub span: Span<()>,
  pub var: Option<String>,
  pub value: IR2Expr,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2IfStmt {
  pub span: Span<()>,
  pub expr: IR2Expr,
  pub vars: Vec<IR2VarDecl>,
  pub body1: Vec<IR2Stmt>,
  pub body2: Vec<IR2Stmt>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2WhileStmt {
  pub span: Span<()>,
  pub expr: IR2Expr,
  pub vars: Vec<IR2VarDecl>,
  pub body: Vec<IR2Stmt>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2Expr {
  Const(Integer),
  Var(IR2VarDecl),
  FuncCall(IR2FuncCall),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2FuncCall {
  pub decl: IR2FuncDecl,
  pub args: Vec<IR2Expr>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2FuncDecl {
  pub name: String,
  pub return_ty: IR2Type,
  pub params: Vec<IR2VarDecl>
}

impl Display for IR2FuncDecl {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}", self.name)?;
    for arg in &self.params {
      write!(f, " ({} {})", arg.name, arg.ty)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2VarDecl {
  pub name: String,
  pub ty: IR2Type,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2Type {
  Int(IR2IntType),
  Ptr(Box<IR2Type>),
}

impl Display for IR2Type {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      IR2Type::Int(ty) => write!(f, "{}", ty),
      IR2Type::Ptr(inner) => write!(f, "*{}", inner),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2IntType {
  I8, I16, I32, I64,
  U8, U16, U32, U64
}

impl Display for IR2IntType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      IR2IntType::I8 => write!(f, "i8"),
      IR2IntType::I16 => write!(f, "i16"),
      IR2IntType::I32 => write!(f, "i32"),
      IR2IntType::I64 => write!(f, "i64"),
      IR2IntType::U8 => write!(f, "u8"),
      IR2IntType::U16 => write!(f, "u16"),
      IR2IntType::U32 => write!(f, "u32"),
      IR2IntType::U64 => write!(f, "u64"),
    }
  }
}

impl IR2IntType {
  pub fn parse(s: &str) -> Option<IR2IntType> {
    Some(match s {
      "i8" => IR2IntType::I8,
      "i16" => IR2IntType::I16,
      "i32" => IR2IntType::I32,
      "i64" => IR2IntType::I64,
      "u8" => IR2IntType::U8,
      "u16" => IR2IntType::U16,
      "u32" => IR2IntType::U32,
      "u64" => IR2IntType::U64,
      &_ => return None
    })
  }
}