use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Sub;

ref_binop!(impl Vari for SubRefVarDataVari,
           impl Sub, sub for Var, Real,
           with val = |val: Real, b: Real| val - b,
           derivative = |adj, _avi_adj, _avi_val, _b| adj);

ref_binop!(impl Vari for SubRefDataVarVari,
           impl Sub, sub for Real, Var,
           with val = |val: Real, b: Real| b - val,
           derivative = |adj: Real, _avi_adj, _avi_val, _b| -adj);

ref_binop!(impl Vari for SubRefVarVarVari,
           impl Sub, sub for Var, Var,
           with val = |a: Real, b: Real| a - b,
           derivative a = |adj: Real, _avi_adj, _avi_val, _bvi_adj, _bvi_val| adj,
           derivative b = |adj: Real, _avi_adj, _avi_val, _bvi_adj, _bvi_val| -adj);

var_binop!(impl Vari for SubVarDataVari,
           impl Sub, sub for Var, Real,
           with val = |val: Real, b: Real| val - b,
           derivative = |adj, _avi_adj, _avi_val, _b| adj);

var_binop!(impl Vari for SubDataVarVari,
           impl Sub, sub for Real, Var,
           with val = |val: Real, b: Real| b - val,
           derivative = |adj: Real, _avi_adj, _avi_val, _b| -adj);

var_binop!(impl Vari for SubVarVarVari,
           impl Sub, sub for Var, Var,
           with val = |a: Real, b: Real| a - b,
           derivative a = |adj: Real, _avi_adj, _avi_val, _bvi_adj, _bvi_val| adj,
           derivative b = |adj: Real, _avi_adj, _avi_val, _bvi_adj, _bvi_val| -adj);


#[cfg(test)]
mod sub_test {
    use super::*;
    use core::memory::*;

    #[test]
    fn v_sub_v() {
        use core::constants::*;
        let x: Real = 3.6;
        let y: Real = 3.0;
        let z: Real = 8.0;
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let vx = Var::from((x, stack.clone()));
        let vy = Var::from((y, stack.clone()));
        let vz = Var::from((z, stack.clone()));
        let v = &vx + &vy;
        let res = &vz - &v;
        res.grad();
        assert_eq!(v.adj(), -ONE);
        assert_eq!(vz.adj(), ONE);
        assert_eq!(vx.adj(), -ONE);
        assert_eq!(vy.adj(), -ONE);
    }

}
