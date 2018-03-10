//! `dorset`, a rust implementation of `Stan Math` library.
//!  The project is not intended to be an exact port of `Math`,
//!  but as a proof of concept to take advantage of some
//!  Rust features in automatic differentiation.
//!  Hence the crate is meant to be experimental.
//!
//! Below is how to do the problem example in section 2.1 of
//! `Bob Carpenter, Matthew D. Hoffman, Marcus Brubaker, Daniel Lee, Peter Li, and Michael J. Betancourt. 2015. The Stan Math Library: Reverse-Mode Automatic Differentiation in C++. arXiv 1509.07164`, using `dorset`.
//! ```
//! #[macro_use]
//! extern crate dorset;
//! use dorset::*;
//!
//! fn main() {
//!     let y: Real = 1.3;
//!     let s = cstack!();
//!     let (mu, sigma) = (var!(s, 0.5), var!(s, 1.2));
//!     let mut lp = var!(s);
//!     lp = &lp - 0.5 * (2.0 * PI).ln();
//!     lp = &lp - ln(&sigma);
//!     lp = &lp - 0.5 * pow((y - &mu) / &sigma, 2.0);
//!     lp.grad();
//!     println!("f(mu, sigma) = {:?}", lp.val());
//!     println!("d.f/d.mu = {:?}", mu.adj());
//!     println!("d.f/d.sigma = {:?}", sigma.adj());
//! }
//!
//! ```
//! The output from `cargo run` of the above code is
//!
//! ```
//! f(mu, sigma) = -1.323482
//! d.f/d.mu = 0.555556
//! d.f/d.sigma = -0.462963
//! ```

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
