use std::fmt;
use core::types::*;
use core::vari::*;
use core::memory::*;
use std::convert::From;
use std::rc::Rc;
use operations;

pub struct Var {
    pub vi_: Rc<*mut Vari>,
}

impl Var {
    pub fn new(vi: Rc<*mut Vari>) -> Var {
        Var { vi_: vi }
    }
    pub fn val(&self) -> Real {
        let vi: &Vari = self.vi_.clone().into();
        vi.val()
    }
    pub fn adj(&self) -> Real {
        let vi: &Vari = self.vi_.clone().into();
        vi.adj()
    }
    pub fn grad(&self) {
        operations::grad::grad(self.vi_.clone());
    }
    pub fn get_vari_refmut<'a>(&self) -> &'a mut Vari{
        self.vi_.clone().into()
    }
    pub fn get_vari_ref<'a>(&self) -> &'a Vari{
        self.vi_.clone().into()
    }
    pub fn set_zero_all_adjoints(&self) {
        let mem = self.get_vari_ref().mem();
        mem.borrow().set_zero_all_adjoints();
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Var {{ val: {}, adj: {} }}", self.val(), self.adj())
    }
}

