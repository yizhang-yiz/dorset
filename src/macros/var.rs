/// Constructs a new `Var` from a real number, using memory arena;
#[macro_export]
macro_rules! var {
    ( $mem:expr ) => {
        {
            let vi = $mem.borrow_mut().alloc(
                Vari::new(0.0, Operand::None,
                           Operand::None, Box::new(do_nothing), $mem.clone()));
            Var::new(vi)
        }
    };
    ( $arena:expr, $val:expr ) => {
        {
            let vi = $arena.borrow_mut().alloc(
                Vari::new($val, Operand::None,
                           Operand::None, Box::new(do_nothing), $arena.clone()));
            Var::new(vi)
        }
    };
}
