use typed_arena;
use core::vari::*;

pub struct ChainStack{
    pub arena: typed_arena::Arena<Vari>,
    pub stack: Vec<*mut Vari>
}
impl ChainStack {
    pub fn new() -> ChainStack {
        ChainStack{
            arena: typed_arena::Arena::new(),
            stack: vec![]
        }
    }
    pub fn alloc(&mut self, vi: Vari) -> *mut Vari {
        let v: *mut Vari = self.arena.alloc(vi);
        self.stack.push(v.clone());
        v
    }
    pub fn chain(&self) {
        for it in self.stack.iter().rev() {
            let vi: &mut Vari = it.clone().into();
            vi.chain();
        }
    }
}
