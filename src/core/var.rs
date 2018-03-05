use std::fmt;
use core::types::*;
use core::vari::*;
use core::memory::*;
use std::convert::From;
use std::cell::RefCell;
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn new() {
//         let stack = Rc::new(RefCell::new(VarStack::new()));
//         let v1 = Var::from((3.0, stack.clone()));
//         let v2 = Var::from((4.0, stack.clone()));
//         assert_eq!(v1.val(), 3.0);
//         assert_eq!(v1.adj(), 0.0);
//         assert_eq!(v2.val(), 4.0);
//         assert_eq!(v2.adj(), 0.0);
//     }

//     #[test]
//     fn grad() {
//         let stack = Rc::new(RefCell::new(VarStack::new()));
//         let v1 = Var::from((3.0, stack.clone()));
//         v1.grad();
//         // grad(&v1);
//         assert_eq!(v1.adj(), 1.0);
//     }
// }
