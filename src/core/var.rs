use std::fmt;
use core::types::*;
use core::vari::*;
use core::memory::*;
use std::convert::From;
use std::cell::RefCell;
use std::rc::Rc;
use operations;

#[derive(Clone)]
pub struct Var {
    mem_: Rc<RefCell<VarStack>>,
    vi_: Rc<RefCell<Vari>>,
}

impl Var {
    pub fn new(mem: Rc<RefCell<VarStack>>,
               vi: Rc<RefCell<Vari>>) -> Var {
        mem.borrow_mut().push(vi.clone());
        Var {
            mem_: mem,
            vi_: vi
        }
    }

    pub fn val(&self) -> Real {
        (*self.vi_).borrow().val()
    }

    pub fn adj(&self) -> Real {
        (*self.vi_).borrow().adj()
    }

    pub fn vi(&self) -> Rc<RefCell<Vari>>{
        self.vi_.clone()
    }

    pub fn storage(&self) -> Rc<RefCell<VarStack>>{
        self.mem_.clone()
    }

    pub fn grad(&self) {
        let stack = self.storage();
        let vi = self.vi();
        operations::grad::grad(vi, stack);
    }

}

/// Construct new `Var` from `Real` and storage
///
impl From<(Real, Rc<RefCell<VarStack>>)> for Var {
    fn from(vs: (Real, Rc<RefCell<VarStack>>) ) -> Var {
    let vi = Rc::new(RefCell::new(VariData::new(vs.0)));
    vs.1.borrow_mut().push(vi.clone());
        // vs.1.push(vi);
        Var {
            mem_: vs.1,
            vi_: vi
        }
    }
}


impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Var {{ val: {}, adj: {} }}", self.val(), self.adj())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v1 = Var::from((3.0, stack.clone()));
        let v2 = Var::from((4.0, stack.clone()));
        assert_eq!(v1.val(), 3.0);
        assert_eq!(v1.adj(), 0.0);
        assert_eq!(v2.val(), 4.0);
        assert_eq!(v2.adj(), 0.0);
    }

    #[test]
    fn grad() {
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v1 = Var::from((3.0, stack.clone()));
        v1.grad();
        // grad(&v1);
        assert_eq!(v1.adj(), 1.0);
    }
}
