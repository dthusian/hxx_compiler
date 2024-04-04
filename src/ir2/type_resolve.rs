use crate::ir2::model::{IR2Expr, IR2Func, IR2FuncDecl, IR2Type};

/// The inferred type of a value. This is needed because we don't know the type
/// the user wanted an integer constant to be.
pub enum ResolveType {
  IntConstant,
  Type(IR2Type)
}

/// Checks if a function's parameters match all provided ResolveTypes
pub fn function_matches(funcs: &IR2FuncDecl, arg_types: &[ResolveType]) -> bool {
  funcs.params.iter()
    .zip(arg_types.iter())
    .all(|(a, b)|
       match b {
         ResolveType::IntConstant => {
           if let IR2Type::Int(_) = a.ty { true } else { false }
         }
         ResolveType::Type(ty) => {
           a.ty == *ty
         }
       }
    )
}

/// Infers the ResolveType of an expression
pub fn infer_expr_type(expr: &IR2Expr) -> ResolveType {
  match expr {
    IR2Expr::Const(_) => ResolveType::IntConstant,
    IR2Expr::Var(var) => ResolveType::Type(var.ty.clone()),
    IR2Expr::FuncCall(func) => ResolveType::Type(func.decl.return_ty.clone())
  }
}