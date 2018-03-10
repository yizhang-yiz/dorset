#[macro_export]
macro_rules! cstack {
    () => {
        {
            Rc::new(RefCell::new(ChainStack::new()))
        }  
    };
}
