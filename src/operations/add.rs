use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Add;

ref_binop!(impl Vari for AddRefVarDataVari,
           impl Add, add for Var, Real,
           with val = |val, b| val + b,
           derivative = |adj, _avi_adj, _avi_val, _b| adj);

ref_binop!(impl Vari for AddRefDataVarVari,
           impl Add, add for Real, Var,
           with val = |val, b| val + b,
           derivative = |adj, _avi_adj, _avi_val, _b| adj);

ref_binop!(impl Vari for AddRefVarVarVari,
           impl Add, add for Var, Var,
           with val = |a, b| a + b,
           derivative a = |adj, _avi_adj, _avi_val, _bvi_adj, _bvi_val| adj,
           derivative b = |adj, _avi_adj, _avi_val, _bvi_adj, _bvi_val| adj);

var_binop!(impl Vari for AddVarDataVari,
           impl Add, add for Var, Real,
           with val = |val, b| val + b,
           derivative = |adj, _avi_adj, _avi_val, _b| adj);

var_binop!(impl Vari for AddDataVarVari,
           impl Add, add for Real, Var,
           with val = |val, b| val + b,
           derivative = |adj, _avi_adj, _avi_val, _b| adj);

var_binop!(impl Vari for AddVarVarVari,
           impl Add, add for Var, Var,
           with val = |a, b| a + b,
           derivative a = |adj, _avi_adj, _avi_val, _bvi_adj, _bvi_val| adj,
           derivative b = |adj, _avi_adj, _avi_val, _bvi_adj, _bvi_val| adj);


#[cfg(test)]
mod add_test {
    use super::*;
    use core::memory::*;

    #[test]
    fn add_d() {
        let x: Real = 3.6;
        let d: Real = 3.0;        
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v = Var::from((d.clone(), stack.clone()));
        let res = &v + x;
        assert_eq!(v.val(), d);
        assert_eq!(v.adj(), 0.0);
        res.grad();
        assert_eq!(v.adj(), 1.0);
    }

    #[test]
    fn d_add() {
        let x: Real = 3.6;
        let d: Real = 3.0;        
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v = Var::from((d.clone(), stack.clone()));
        let res = x + &v;
        assert_eq!(v.val(), d);
        assert_eq!(v.adj(), 0.0);
        res.grad();
        assert_eq!(v.adj(), 1.0);
    }

    #[test]
    fn v_add_v() {
        let x: Real = 3.6;
        let y: Real = 3.0;
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v1 = Var::from((x, stack.clone()));
        let v2 = Var::from((y, stack.clone()));
        let res = &v1 + &v2;
        assert_eq!(res.val(), x + y);
        assert_eq!(res.adj(), 0.0);
        res.grad();
        assert_eq!(v1.adj(), 1.0);
        assert_eq!(v2.adj(), 1.0);
    }

}
