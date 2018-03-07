use std::fmt;
use core::types::*;
use core::vari::*;
use core::memory::*;
use std::convert::From;
use operations;

pub struct Var {
    pub vi_: *mut Vari,
}

impl Var {
    pub fn new(vi: *mut Vari) -> Var {
        Var { vi_: vi }
    }
    pub fn val(&self) -> Real {
        let v: &Vari = self.vi_.clone().into();
        v.val()
    }
    pub fn adj(&self) -> Real {
        let v: &Vari = self.vi_.clone().into();
        v.adj()
    }
    pub fn grad(&self) {
        operations::grad::grad(self.vi_.clone());
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Var {{ val: {}, adj: {} }}", self.val(), self.adj())
    }
}

