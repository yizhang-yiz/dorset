use std::fmt::Debug;

pub trait Chainable: Debug {
    // fn chain(&mut self);
    // fn init_dependent(&mut self);
    // fn set_zero_adjoint(&mut self);

    // #[inline]
    
        
}

pub struct ChainableStackStorage<'a> {
    var_stack_: Vec<Box<Chainable + 'a>>,
//     var_alloc_stack_: Vec<Box<ChainableAlloc + 'b>>,
}

impl<'a> ChainableStackStorage<'a> {
    pub fn new(var_stack: Vec<Box<Chainable + 'a>> ) -> ChainableStackStorage<'a>{
        ChainableStackStorage {
            var_stack_: var_stack            
        }
    }

    pub fn stack(&'a self) -> &Vec<Box<Chainable + 'a>>{
        &self.var_stack_
    }

    pub fn push<T: Chainable + 'a>(&mut self, x: T) {
        self.var_stack_.push(Box::new(x));
    }
}
