//! IR2 is a stripped-down version of IR1 where function overloads and types are resolved.
//! After successful generation of IR2, all programs are valid at a language level.
//! Detailed span information is also lost at this stage.
//!
//! Transformations done to reach IR2:
//! - Function overload resolution
//! - Type parsing
//! - Variable renaming and hoisting (to avoid conflicts with rebinding)
//! - Conversion of function call statements into unbound assignments
//!

pub mod model;
pub mod type_resolve;