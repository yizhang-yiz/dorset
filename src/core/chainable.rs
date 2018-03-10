use std::fmt::Debug;

#[allow(dead_code)]
pub trait Chainable: Debug {
    fn chain(&mut self) {}
    fn init_dependent(&mut self);
    fn set_zero_adjoint(&mut self);

    // #[inline]
}

#[allow(dead_code)]
pub struct ChainableStackStorage<'a> {
    var_stack_: Vec<Box<Chainable + 'a>>,
    //     var_alloc_stack_: Vec<Box<ChainableAlloc + 'b>>,
    // mem_: MemArena,
}

#[allow(dead_code)]
impl<'a> ChainableStackStorage<'a> {
    pub fn new(var_stack: Vec<Box<Chainable + 'a>>) -> ChainableStackStorage<'a> {
        ChainableStackStorage {
            var_stack_: var_stack, // mem_: MemArena::new
        }
    }

    pub fn stack(&'a self) -> &Vec<Box<Chainable + 'a>> {
        &self.var_stack_
    }

    pub fn push<T: Chainable + 'a>(&mut self, x: T) {
        self.var_stack_.push(Box::new(x));
    }
}
