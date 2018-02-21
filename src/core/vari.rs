use core::types::*;
use core::constants::*;
use core::chainable::*;

pub trait Vari: Chainable {
    fn new(v: Real) -> Self;
    // fn new<'a>(s: &'a mut ChainableStackStorage, v: Real) -> Self where Self: Sized {
    //     let mut x: Self = Self::new_(v);
    //     s.push()
    //     x
    // }
    fn val(&self) -> Real;
    fn adj(&self) -> Real;
    fn set_adj(&mut self, v: Real);
    fn init_dependent(&mut self) {
        self.set_adj(ONE);
    }
    fn set_zero_adjoint(&mut self) {
        self.set_adj(ZERO);        
    }
}

