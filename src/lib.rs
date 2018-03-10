//! `Dorset`, a rust implementation of `Stan Math` library.
//!  The project is not intended to be an exact port of `Math`,
//!  but as a proof of concept to take advantage of some
//!  Rust features in automatic differentiation.
//!  Hence the crate is meant to be experimental.

extern crate typed_arena;

extern crate float_cmp;

#[allow(unused_macros)]
#[macro_use]
pub mod macros;
// pub mod macros;

#[macro_use]
pub mod core;
pub use core::*;

#[allow(unused_imports)]
#[macro_use]
pub mod operations;
pub use operations::*;

pub use std::rc::Rc;
pub use std::cell::RefCell;
