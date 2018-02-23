use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;

var_uniop!(impl Vari for CosVari,
           cos for Var,
           with val = cos,
           derivative = |adj: Real, _vi_adj, vi_val: Real| -adj * vi_val.sin());

pub fn cos(v: &Var) -> Var{
    v.cos()
}

#[cfg(test)]
mod log_test {
    use super::*;
    use core::memory::*;

    // #[test]
    // fn cos_add_sin() {
    //     use core::constants::*;
    //     let x: Real = 3.0;
    //     let y: Real = 7.0;
    //     let stack = Rc::new(RefCell::new(VarStack::new()));
    //     let v1 = Var::from((x, stack.clone()));
    //     let v2 = Var::from((y, stack.clone()));
    //     let res = sin(&v1) + sin(&v2);
    //     assert_eq!(v1.adj(), ZERO);
    //     assert_eq!(v2.adj(), ZERO);
    //     res.grad();
    //     assert_eq!(v1.adj(), x.cos());
    //     assert_eq!(v2.adj(), y.cos());
    // }
}
