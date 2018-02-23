use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;

var_uniop!(impl Vari for SinVari,
           sin for Var,
           with val = sin,
           derivative = |adj, _vi_adj, vi_val: Real| adj * vi_val.cos());

pub fn sin(v: &Var) -> Var{
    v.sin()
}

#[cfg(test)]
mod log_test {
    use super::*;
    use core::memory::*;

    #[test]
    fn sin_v() {
        let x: Real = 3.0;
        let dsinx: Real = x.cos();
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v = Var::from((3.0, stack.clone()));
        let res = v.sin();
        res.grad();
        assert_eq!(v.adj(), dsinx);
    }

    #[test]
    fn sin_add_d() {
        use core::constants::*;
        let x: Real = 3.0;
        let y: Real = 7.0;
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v1 = Var::from((x, stack.clone()));
        let res = sin(&v1) + y;
        assert_eq!(v1.adj(), ZERO);
        res.grad();
        assert_eq!(v1.adj(), x.cos());
    }

    #[test]
    fn d_add_sin() {
        use core::constants::*;
        let x: Real = 3.0;
        let y: Real = 7.0;
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v1 = Var::from((x, stack.clone()));
        let res = y + sin(&v1);
        assert_eq!(v1.adj(), ZERO);
        res.grad();
        assert_eq!(v1.adj(), x.cos());
    }

    #[test]
    fn sin_add_sin() {
        use core::constants::*;
        let x: Real = 3.0;
        let y: Real = 7.0;
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v1 = Var::from((x, stack.clone()));
        let v2 = Var::from((y, stack.clone()));
        let res = sin(&v1) + sin(&v2);
        assert_eq!(v1.adj(), ZERO);
        assert_eq!(v2.adj(), ZERO);
        res.grad();
        assert_eq!(v1.adj(), x.cos());
        assert_eq!(v2.adj(), y.cos());
    }
}
