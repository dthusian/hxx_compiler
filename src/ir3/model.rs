
pub type IR3VarID = u32;
pub type IR3BBID = u32;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR3Type {
  Data(u32),
  Ptr
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IR3Op {
  Add(IR3DataBinaryOp),
  Sub(IR3DataBinaryOp),
}

impl IR3Op {
  pub fn returns(&self) -> Vec<IR3VarID> {
    todo!()
  }
  
  pub fn depends_on(&self) -> Vec<IR3VarID> {
    todo!()
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3DataBinaryOp {
  ty: IR3Type,
  a: IR3VarID,
  b: IR3VarID,
  q: IR3VarID,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3BasicBlock {
  id: IR3BBID,
  instructions: Vec<IR3Op>,
  ending: IR3EndOp
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IR3Function {
  args: Vec<IR3Type>,
  basic_blocks: Vec<IR3BasicBlock>
}