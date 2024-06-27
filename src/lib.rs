//! A crate of utilities for working with the WebIDL AST by extending `weedle2`.
//! The crate's API is not currently stable yet.

mod extend;
mod symbol;
pub use extend::*;
pub use symbol::*;
