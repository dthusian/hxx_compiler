use std::fmt::{Display, Formatter, Write};
use crate::common::util::join;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3Function {
  name: String,
  args: Vec<IR3Type>,
  ret: IR3Type,
  basic_blocks: Vec<IR3BasicBlock>,
  attrs: Vec<(IR3FunctionAttr, String)>
}

impl Display for IR3Function {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "ir3function {}", &self.name)?;
    if !self.attrs.is_empty() {
      let attr_str = self.attrs.iter()
        .map(|(attr, val)| format!("{}={}", attr, val))
        .collect::<Vec<_>>()
        .join(" ");
      write!(f, " attrs {}", attr_str)?;
    }
    if !self.args.is_empty() {
      write!(f, " args {}", join(&self.args, " "))?;
    }
    write!(f, " returns {} {{\n{}\n}}", &self.ret, join(&self.basic_blocks, "\n"))?;
    Ok(())
  }
} 

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3BasicBlock {
  id: IR3BBID,
  instructions: Vec<IR3Op>,
  ending: IR3EndOp
}

impl Display for IR3BasicBlock {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "@{}:\n  {}\n  {}", self.id, join(&self.instructions, "\n  "), self.ending)
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3Op {
  kind: IR3OpKind,
  ty: IR3Type,
  input: Vec<IR3VarID>,
  output: Vec<IR3VarID>
}

impl Display for IR3Op {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    fn fmt1(f: &mut Formatter<'_>, s: &IR3Op) -> std::fmt::Result {
      write!(f, "${} = {} {} ${}", join(&s.output, " $"), s.kind.name(), s.ty, join(&s.input, " $"))
    }
    fn fmt2(f: &mut Formatter<'_>, s: &IR3Op, opt: impl Display) -> std::fmt::Result {
      write!(f, "${} = {} {} {} ${}", join(&s.output, " $"), s.kind.name(), s.ty, opt, join(&s.input, " $"))
    }
    fn fmt3(f: &mut Formatter<'_>, s: &IR3Op, call_info: &IR3Call) -> std::fmt::Result {
      write!(
        f, "${} = {} {} {} {}", join(&s.output, " $"), s.kind.name(), s.ty, &call_info.symbol_name,
        call_info.arg_types
          .iter()
          .zip(s.input.iter())
          .map(|(ty, var)| format!("{} {}", ty, var))
          .collect::<Vec<String>>()
          .join(" ")
      )
    }
    fn fmt4(f: &mut Formatter<'_>, s: &IR3Op, phi: &IR3Phi) -> std::fmt::Result {
      write!(
        f, "${} = {} {} {}", join(&s.output, " $"), s.kind.name(), s.ty,
        phi.blocks
          .iter()
          .zip(s.input.iter())
          .map(|(block, var)| format!("{} {}", var, block))
          .collect::<Vec<String>>()
          .join(" ")
      )
    }
    match &self.kind {
      IR3OpKind::Cmp(mode) => fmt2(f, self, mode),
      IR3OpKind::Const(v) => fmt2(f, self, v),
      IR3OpKind::Sext(ty2) | IR3OpKind::Zext(ty2) => fmt2(f, self, ty2),
      IR3OpKind::Arg(idx) => fmt2(f, self, idx),
      IR3OpKind::Call(call) => fmt3(f, self, &call),
      IR3OpKind::Phi(phi) => fmt4(f, self, &phi),
      _ => fmt1(f, self),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR3OpKind {
  Add,
  Sub,
  Cmp(IR3CompareMode),
  And,
  Or,
  Xor,
  Not,
  Sll,
  Srl,
  Sra,
  
  Smull,
  Umull,
  Smulh,
  Umulh,
  Sdiv,
  Udiv,
  Srem,
  Urem,
  
  PtrLoad,
  PtrStore,
  PtrUadd,
  PtrSadd,
  
  Const(u64),
  Sext(IR3Type),
  Zext(IR3Type),
  
  Arg(u32),
  Call(IR3Call),
  Phi(IR3Phi),
}

impl IR3Op {
  pub fn arg_type(&self, idx: usize) -> IR3Type {
    match &self.kind {
      IR3OpKind::Call(call_info) => call_info.arg_types[idx],
      _ => self.ty,
    }
  }
  
  pub fn return_type(&self) -> IR3Type {
    match &self.kind {
      IR3OpKind::Cmp(_) => IR3Type::Data(1),
      IR3OpKind::PtrSadd | IR3OpKind::PtrUadd => IR3Type::Ptr,
      IR3OpKind::PtrStore => IR3Type::Void,
      _ => self.ty
    }
  }
}

impl IR3OpKind {
  pub fn name(&self) -> &'static str {
    match self {
      IR3OpKind::Add => "add",
      IR3OpKind::Sub => "sub",
      IR3OpKind::Cmp(_) => "cmp",
      IR3OpKind::And => "and",
      IR3OpKind::Or => "or",
      IR3OpKind::Xor => "xor",
      IR3OpKind::Not => "not",
      IR3OpKind::Sll => "sll",
      IR3OpKind::Srl => "srl",
      IR3OpKind::Sra => "sra",
      IR3OpKind::Smull => "smull",
      IR3OpKind::Umull => "umull",
      IR3OpKind::Smulh => "smulh",
      IR3OpKind::Umulh => "umulh",
      IR3OpKind::Sdiv => "sdiv",
      IR3OpKind::Udiv => "udiv",
      IR3OpKind::Srem => "srem",
      IR3OpKind::Urem => "urem",
      IR3OpKind::PtrLoad => "ptr_load",
      IR3OpKind::PtrStore => "ptr_store",
      IR3OpKind::PtrUadd => "ptr_uadd",
      IR3OpKind::PtrSadd => "ptr_sadd",
      IR3OpKind::Const(_) => "const",
      IR3OpKind::Sext(_) => "sext",
      IR3OpKind::Zext(_) => "zext",
      IR3OpKind::Arg(_) => "arg",
      IR3OpKind::Call(_) => "call",
      IR3OpKind::Phi(_) => "phi"
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR3EndOp {
  Br {
    block: IR3BBID
  },
  BrIf {
    block1: IR3BBID,
    block2: IR3BBID,
    cond: IR3VarID
  },
  Ret {
    ty: IR3Type,
    var: IR3VarID
  }
}

impl Display for IR3EndOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      IR3EndOp::Br { block } => {
        write!(f, "br @{}", block)
      }
      IR3EndOp::BrIf { block1, block2, cond } => {
        write!(f, "br_if ${} @{} @{}", cond, block1, block2)
      }
      IR3EndOp::Ret { ty, var } => {
        write!(f, "ret {} ${}", ty, var)
      }
    }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR3CompareMode {
  ULt,
  UGt,
  ULe,
  UGe,
  SLt,
  SGt,
  SLe,
  SGe,
  Eq,
  Ne
}

impl IR3CompareMode {
  pub fn name(self) -> &'static str {
    match self {
      IR3CompareMode::ULt => "ult",
      IR3CompareMode::UGt => "ugt",
      IR3CompareMode::ULe => "ule",
      IR3CompareMode::UGe => "uge",
      IR3CompareMode::SLt => "slt",
      IR3CompareMode::SGt => "sgt",
      IR3CompareMode::SLe => "sle",
      IR3CompareMode::SGe => "sge",
      IR3CompareMode::Eq => "eq",
      IR3CompareMode::Ne => "ne"
    }
  }
}

impl Display for IR3CompareMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name())
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3Call {
  symbol_name: String,
  arg_types: Vec<IR3Type>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3Phi {
  blocks: Vec<IR3BBID>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR3FunctionAttr {
  BuiltinFunction
}

impl IR3FunctionAttr {
  pub fn name(self) -> &'static str {
    match self {
      IR3FunctionAttr::BuiltinFunction => "builtin-function"
    }
  }
}

impl Display for IR3FunctionAttr {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name())
  }
}

pub type IR3VarID = u32;
pub type IR3BBID = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR3Type {
  Data(u32),
  Ptr,
  Void
}

impl Display for IR3Type {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      IR3Type::Data(w) => write!(f, "d{}", w),
      IR3Type::Ptr => write!(f, "ptr"),
      IR3Type::Void => write!(f, "void"),
    }
  }
}