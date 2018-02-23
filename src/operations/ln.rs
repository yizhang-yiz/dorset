use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;

var_uniop!(impl Vari for LogVari,
           ln for Var,
           with val = ln,
           derivative = |adj, _vi_adj, vi_val| adj / vi_val);

pub fn ln(v: &Var) -> Var{
    v.ln()
}


#[cfg(test)]
mod log_test {
    use super::*;
    use core::memory::*;

    #[test]
    fn log_v() {
        let d: Real = 3.0;
        let dlogd: Real = 1.0/d;
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v = Var::from((3.0, stack.clone()));
        let res = v.ln();
        res.grad();
        // grad(&res);
        assert_eq!(v.adj(), dlogd);
    }

    #[test]
    fn log_log_v() {
        use float_cmp::*;
        let x: Real = 3.0;
        let logx: Real = x.ln();
        let dloglogx: Real = 1.0/(logx * x);
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let v = Var::from((3.0, stack.clone()));
        let res = v.ln().ln();
        res.grad();
        assert!(v.adj().approx_eq_ulps(&dloglogx,2));
    }

    #[test]
    fn log_add() {
        use float_cmp::*;
        use core::constants::*;
        let x: Real = 3.0;
        let y: Real = 4.0;
        let z: Real = 5.0;
        let stack = Rc::new(RefCell::new(VarStack::new()));
        let vx = Var::from((x, stack.clone()));
        let vy = Var::from((y, stack.clone()));
        let vz = Var::from((z, stack.clone()));
        let res = ln(&vx) + ln(&vy) + ln(&vz);
        res.grad();
        assert!(vx.adj().approx_eq_ulps(&(1.0/x),2));
        assert!(vy.adj().approx_eq_ulps(&(1.0/y),2));
        assert!(vz.adj().approx_eq_ulps(&(1.0/z),2));
        stack.borrow().set_zero_all_adjoints();
        assert_eq!(vx.adj(), ZERO);
        assert_eq!(vy.adj(), ZERO);
        assert_eq!(vz.adj(), ZERO);
    }
}
