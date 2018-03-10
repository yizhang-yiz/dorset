use super::*;
pub use std::ops::Add;

fn chain_addition(vi: &Vari) {
    let adj = vi.adj();
    match (vi.a.clone(), vi.b.clone()) {
        (Operand::Vari(ptr), Operand::Data(_bd)) => {
            let avi: &mut Vari = ptr.clone().into();
            let avi_adj = avi.adj();
            avi.set_adj(avi_adj + adj);
        }
        (Operand::Data(_ad), Operand::Vari(ptr)) => {
            let bvi: &mut Vari = ptr.clone().into();
            let bvi_adj = bvi.adj();
            bvi.set_adj(bvi_adj + adj);
        }
        (Operand::Vari(ap), Operand::Vari(bp)) => {
            let avi: &mut Vari = ap.clone().into();
            let avi_adj = avi.adj();
            avi.set_adj(avi_adj + adj);
            let bvi: &mut Vari = bp.clone().into();
            let bvi_adj = bvi.adj();
            bvi.set_adj(bvi_adj + adj);
        }
        _ => {}
    }
}

binop!(impl Add, add
       for Var, Real, |x, y| x + y,
       for Real, Var, |x, y| x + y,
       for Var, Var, |x, y| x + y,
       chain Fn = chain_addition);

#[cfg(test)]
mod test {
    use super::*;
    use core::memory::*;

    #[test]
    fn add() {
        let mut x: Real = 3.6;
        let mut y: Real = 3.0;        
        let stack = Rc::new(RefCell::new(ChainStack::new()));
        let vx = var!(stack, x);
        let vy = var!(stack, y);
        let mut v = &vx + &vy;
        v.grad();
        assert_eq!(v.val(), x + y);
        assert_eq!(vx.adj(), 1.0);
        assert_eq!(vy.adj(), 1.0);

        v.set_zero_all_adjoints();
        y = 8.9;
        v = &vx + &y;
        v.grad();
        assert_eq!(v.val(), x + y);
        assert_eq!(vx.adj(), 1.0);
        assert_eq!(vy.adj(), 0.0);

        v.set_zero_all_adjoints();
        x = 83.1;
        v = &x + &vy;
        v.grad();
        assert_eq!(v.val(), x + vy.val());
        assert_eq!(vx.adj(), 0.0);
        assert_eq!(vy.adj(), 1.0);
    }
}
