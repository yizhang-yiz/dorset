use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Mul;

fn chain_multiplication(vi: &Vari) {
    let adj = vi.adj();
    match (vi.a.clone(), vi.b.clone()) {
        (Operand::Vari(ptr), Operand::Data(bd)) => {
            let avi: &mut Vari = ptr.clone().into();
            let avi_val = avi.val();
            let avi_adj = avi.adj();
            avi.set_adj(avi_adj + adj * bd);
        }
        (Operand::Data(ad), Operand::Vari(ptr)) => {
            let bvi: &mut Vari = ptr.clone().into();
            let bvi_val = bvi.val();
            let bvi_adj = bvi.adj();
            bvi.set_adj(bvi_adj + adj * ad);
        }
        (Operand::Vari(ap), Operand::Vari(bp)) => {
            let avi: &mut Vari = ap.clone().into();
            let avi_val = avi.val();
            let avi_adj = avi.adj();
            let bvi: &mut Vari = bp.clone().into();
            let bvi_val = bvi.val();
            let bvi_adj = bvi.adj();
            avi.set_adj(avi_adj + adj * bvi_val);
            bvi.set_adj(bvi_adj + adj * avi_val);
        }
        _ => {}
    }
}

binop!(impl Mul, mul
       for Var, Real, |x, y| x * y,
       for Real, Var, |x, y| x * y,
       for Var, Var, |x, y| x * y,
       chain Fn = chain_multiplication);

#[cfg(test)]
mod test {
    use super::*;
    use core::memory::*;

    #[test]
    fn mul() {
        let mut x: Real = 3.6;
        let mut y: Real = 3.0;        
        let stack = Rc::new(RefCell::new(ChainStack::new()));
        let vx = var!(stack, x);
        let vy = var!(stack, y);
        let mut v = &vx * &vy;
        v.grad();
        assert_eq!(v.val(), x * y);
        assert_eq!(vx.adj(), y);
        assert_eq!(vy.adj(), x);

        v.set_zero_all_adjoints();
        y = 8.9;
        v = &vx * &y;
        v.grad();
        assert_eq!(v.val(), x * y);
        assert_eq!(vx.adj(), y);
        assert_eq!(vy.adj(), 0.0);

        v.set_zero_all_adjoints();
        x = 83.1;
        v = &x * &vy;
        v.grad();
        assert_eq!(v.val(), x * vy.val());
        assert_eq!(vx.adj(), 0.0);
        assert_eq!(vy.adj(), x);
    }
}
