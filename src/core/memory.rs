use core::vari::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Index;

#[derive(Debug)]
pub struct VarStack (Vec<Rc<RefCell<Vari>>>);

impl VarStack {
    pub fn new() -> VarStack {
        VarStack(vec![])
    }

    pub fn push(&mut self, v: Rc<RefCell<Vari>>) {
        self.0.push(v);
    }

    pub fn last(&self) -> Rc<RefCell<Vari>>{
        self.0.last().unwrap().clone()
    }

    pub fn chain(&self) {
        for it in self.0.iter().rev() {
            it.borrow_mut().chain();
        }
    }

    pub fn set_zero_all_adjoints(&self) {
        for it in self.0.iter() {
            it.borrow_mut().set_zero_adjoint();
        }
    }
}

impl Index<usize> for VarStack {
    type Output = Rc<RefCell<Vari>>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// impl fmt::Debug for VarStack {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for vi in self.0 {
//             write!(f, "Var {{ val: {}, adj: {} }}", self.val(), self.adj())
//         }
//     }
// }

// static mut var_stack: Vec<Box<Vari>> = vec![];

// extern crate typed_arena;

// struct MemArena (typed_arena::Arena<u8>);

// impl MemArena {
//     fn with_capacity(size: usize) -> MemArena {
//         MemArena(typed_arena::Arena::with_capacity(size))
//     }
//     fn with_capacity(size: usize) -> MemArena {
//         MemArena(typed_arena::Arena::with_capacity(size))
//     }
// }
// #[test]
// fn test1() {
//     let arena: typed_arena::Arena<u8> = typed_arena::Arena::with_capacity(2);
// }
