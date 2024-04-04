use crate::ir2::model::{IR2FuncDecl, IR2IntType, IR2Type, IR2VarDecl};

pub fn default_builtins() -> Vec<IR2FuncDecl> {
  vec![
    IR2FuncDecl {
      name: "add".to_string(),
      return_ty: IR2Type::Int(IR2IntType::I32),
      params: vec![IR2VarDecl {
        name: "a".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }, IR2VarDecl {
        name: "b".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }],
    },
    IR2FuncDecl {
      name: "sub".to_string(),
      return_ty: IR2Type::Int(IR2IntType::I32),
      params: vec![IR2VarDecl {
        name: "a".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }, IR2VarDecl {
        name: "b".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }],
    },
    IR2FuncDecl {
      name: "mul".to_string(),
      return_ty: IR2Type::Int(IR2IntType::I32),
      params: vec![IR2VarDecl {
        name: "a".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }, IR2VarDecl {
        name: "b".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }],
    },
    IR2FuncDecl {
      name: "lt".to_string(),
      return_ty: IR2Type::Int(IR2IntType::U8),
      params: vec![IR2VarDecl {
        name: "a".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }, IR2VarDecl {
        name: "b".to_string(),
        ty: IR2Type::Int(IR2IntType::I32),
      }]
    },
    IR2FuncDecl {
      name: "println".to_string(),
      return_ty: IR2Type::Void,
      params: vec![IR2VarDecl {
        name: "a".to_string(),
        ty: IR2Type::Int(IR2IntType::U32),
      }],
    }
  ]
}