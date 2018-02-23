use core::vari::*;
use std::cell::RefCell;
use std::rc::Rc;
use core::memory::*;

// pub fn grad(v: &Var) {
pub fn grad(vi: Rc<RefCell<Vari>>, stack: Rc<RefCell<VarStack>>) {
    vi.borrow_mut().init_dependent();
    stack.borrow().chain();
}
