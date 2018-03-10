//! The core data structures for the autodiff expression
//! graph and memory management.

pub mod types;
pub use self::types::*;

pub mod constants;
pub use self::constants::*;

#[macro_use]
pub mod vari;
pub use self::vari::*;

#[macro_use]
pub mod var;
pub use self::var::Var;

mod chainable;

pub mod memory;
pub use self::memory::ChainStack;
