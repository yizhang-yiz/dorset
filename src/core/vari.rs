use core::types::*;
use core::constants::*;
use std::fmt::Debug;

pub trait Vari: Debug {
    fn data(&self) -> &VariData;
    fn data_mut(&mut self) -> &mut VariData;
    fn val(&self) -> Real {
        self.data().val_.clone()
    }
    fn set_val(&mut self, v: Real) {
        self.data_mut().val_ = v;
    }
    fn adj(&self) -> Real {
        self.data().adj_.clone()
    }
    fn set_adj(&mut self, v: Real) {
        self.data_mut().adj_ = v;
    }
    fn init_dependent(&mut self) {
        self.set_adj(ONE);
    }
    fn set_zero_adjoint(&mut self) {
        self.set_adj(ZERO);        
    }
    fn chain(&mut self) {}
}

// impl fmt::Debug for Var {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Var {{ val: {}, adj: {} }}", self.val(), self.adj())
//     }
// }

#[derive(Debug, Clone)]
pub struct VariData{
    pub val_: Real,
    pub adj_: Real,
}

impl VariData {
    pub fn new(v: Real) -> VariData {
        VariData{
            val_: v,
            adj_: 0.0 as Real
        }
    }
}

impl Vari for VariData {
    fn data(&self) -> &VariData {
        self
    }
    fn data_mut(&mut self) -> &mut VariData {
        self
    }
}

impl From<Real> for VariData {
    fn from(v: Real) -> VariData {
        VariData::new(v)
    }
}


// impl Chainable for VariData {
//     fn init_dependent(&mut self) {
//         self.set_adj(ONE);
//     }
//     fn set_zero_adjoint(&mut self) {
//         self.set_adj(ZERO);        
//     }
// }
