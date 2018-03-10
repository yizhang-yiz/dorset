//! Off-the-shelve memory arena using `typed-arena`
//! crate. 
use typed_arena;
pub use core::vari::*;
pub use std::rc::Rc;

pub struct ChainStack{
    pub arena: typed_arena::Arena<Vari>,
    pub stack: Vec<Rc<*mut Vari>>
}
impl ChainStack {
    pub fn new() -> ChainStack {
        ChainStack{
            arena: typed_arena::Arena::new(),
            stack: vec![]
        }
    }
    pub fn alloc(&mut self, vi: Vari) -> Rc<*mut Vari> {
        let v: Rc<*mut Vari> = Rc::new(self.arena.alloc(vi));
        self.stack.push(v.clone());
        v
    }
    pub fn chain(&self) {
        for it in self.stack.iter().rev() {
            let vi: &mut Vari = it.clone().into();
            vi.chain();
        }
    }
    pub fn set_zero_all_adjoints(&self) {
        for it in self.stack.iter() {
            let vi: &mut Vari = it.clone().into();
            vi.set_zero_adjoint();
        }
    }
}
