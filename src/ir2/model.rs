use std::fmt::{Display, Formatter, write};
use rug::{Integer};
use crate::common::err::{Cerr, CerrKind};
use crate::common::span::{Span, SpanPlace};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2Program {
  pub funcs: Vec<IR2Func>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2Func {
  pub span: SpanPlace,
  pub decl: IR2FuncDecl,
  pub scope: IR2Scope
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2Stmt {
  Set(IR2SetStmt),
  If(IR2IfStmt),
  While(IR2WhileStmt),
  Break(SpanPlace)
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2SetStmt {
  pub span: SpanPlace,
  pub var: Option<IR2VarDecl>,
  pub value: IR2Expr,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2IfStmt {
  pub span: SpanPlace,
  pub expr: IR2Expr,
  pub scope1: IR2Scope,
  pub scope2: Option<IR2Scope>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2WhileStmt {
  pub span: SpanPlace,
  pub expr: IR2Expr,
  pub scope: IR2Scope,
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
  pub attrs: Vec<IR2FuncAttr>,
  pub name: String,
  pub return_ty: IR2Type,
  pub params: Vec<IR2VarDecl>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2FuncAttr {
  BuiltinFunction(String)
}

impl Display for IR2FuncDecl {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}", self.name)?;
    for arg in &self.params {
      write!(f, " ({} {})", arg.name, arg.ty)?;
    }
    write!(f, " {})", self.return_ty)?;
    Ok(())
  }
}

/// A scope is a combination of a list of variables and a statement list.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2Scope {
  pub vars: Vec<IR2VarDecl>,
  pub stmt_list: Vec<IR2Stmt>
}

/// This struct is NOT the same as IR1VarDecl.
/// It is the renamed version of an IR1VarDecl.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR2VarDecl {
  pub name: String,
  pub ty: IR2Type,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2Type {
  Void,
  Int(IR2IntType),
  Ptr(Box<IR2Type>),
}

impl Display for IR2Type {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      IR2Type::Void => write!(f, "void"),
      IR2Type::Int(ty) => write!(f, "{}", ty),
      IR2Type::Ptr(inner) => write!(f, "*{}", inner),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR2IntType {
  I8, I16, I32, I64,
  U8, U16, U32, U64,
  Bool,
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
      IR2IntType::Bool => write!(f, "bool")
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
      "bool" => IR2IntType::Bool,
      &_ => return None
    })
  }
}