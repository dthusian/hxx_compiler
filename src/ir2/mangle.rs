use crate::common::util::{map_join};
use crate::ir2::model::{IR2FuncDecl, IR2Type};

pub fn mangle_function(decl: &IR2FuncDecl) -> String {
  format!("_HX${}${}", decl.name, map_join(&decl.params, |v| &v.ty, "$"))
}