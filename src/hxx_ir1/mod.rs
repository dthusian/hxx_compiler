//! IR1 is a basic AST. Minimal effort was taken to convert S-expressions
//! into language constructs. At this point, only syntax errors can be identified.
//!
//! IR1 does not have a defined text format, but it can be printed
//! with the Debug trait.

pub mod to_ir2;
pub mod model;
pub mod from_hxx;