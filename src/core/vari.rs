use core::types::*;
use core::constants::*;
use core::memory::*;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::default::Default;
use std::convert::From;

#[derive(Clone)]
pub enum Operand {
    Vari(Rc<*mut Vari>),
    Data(Real),
    None,
}

pub struct Vari {
    data: VariData,
    pub a: Operand,
    pub b: Operand,
    chain_op: Box<Fn(&Vari)>,
    mem_: Rc<RefCell<ChainStack>>,
}

impl Vari {
    pub fn new(v: Real, oa: Operand, ob: Operand, op: Box<Fn(&Vari)>, mem: Rc<RefCell<ChainStack>>) -> Vari {
        Vari {
            data: VariData::new(v),
            a: oa,
            b: ob,
            chain_op: op,
            mem_: mem,
        }
    }
    pub fn val(&self) -> Real {
        self.data.val_
    }
    pub fn adj(&self) -> Real {
        self.data.adj_
    }
    pub fn set_adj(&mut self, v: Real) {
        self.data.adj_ = v;
    }
    pub fn init_dependent(&mut self) {
        self.set_adj(ONE);
    }
    pub fn set_zero_adjoint(&mut self) {
        self.set_adj(ZERO);
    }
    pub fn chain(&self) {
        (self.chain_op)(self);
    }
    pub fn mem(&self) -> Rc<RefCell<ChainStack>> {
        self.mem_.clone()
    }
}
 
impl<'a> From<Rc<*mut Vari>> for &'a mut Vari {
    fn from(a: Rc<*mut Vari>) -> &'a mut Vari {
        unsafe { &mut (**Rc::into_raw(a)) }
    }
}
impl<'a> From<Rc<*mut Vari>> for &'a Vari {
    fn from(a: Rc<*mut Vari>) -> &'a Vari {
        unsafe { &(**Rc::into_raw(a)) }
    }
}

pub fn do_nothing(v: &Vari) {}

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

impl From<Real> for VariData {
    fn from(v: Real) -> VariData {
        VariData::new(v)
    }
}

