use super::*;

fn chain_pow(vi: &Vari) {
    let val = vi.val();
    let adj = vi.adj();
    match (vi.a.clone(), vi.b.clone()) {
        (Operand::Vari(ptr), Operand::Data(bd)) => {
            let avi: &mut Vari = ptr.clone().into();
            let avi_val = avi.val();
            let avi_adj = avi.adj();
            avi.set_adj(avi_adj + adj * bd * val / avi_val);
        }
        (Operand::Data(ad), Operand::Vari(ptr)) => {
            let bvi: &mut Vari = ptr.clone().into();
            let bvi_adj = bvi.adj();
            bvi.set_adj(bvi_adj + adj * val * ad.ln());
        }
        (Operand::Vari(ap), Operand::Vari(bp)) => {
            let avi: &mut Vari = ap.clone().into();
            let avi_val = avi.val();
            let avi_adj = avi.adj();
            let bvi: &mut Vari = bp.clone().into();
            let bvi_val = bvi.val();
            let bvi_adj = bvi.adj();
            avi.set_adj(avi_adj + adj * bvi_val * val / avi_val);
            bvi.set_adj(bvi_adj + adj * val * avi_val.ln());
        }
        _ => {}
    }
}

pub trait OpPow<Other> {
    type Output;
    fn pow(self, other: Other) -> Self::Output;
}

pub fn pow<S, T: OpPow<S>>(a: T, b: S) -> <T as OpPow<S>>::Output {
    OpPow::pow(a, b)
}

// impl<'a, 'b> OpPow<&'a Real> for &'b Real {
//     type Output = Real;
//     fn pow(self, other: &'a Real) -> Real {
//         self.clone().powf(other.clone())
//     } 
// }

binop!(impl OpPow, pow
       for Var, Real, |x: Real, y: Real| x.powf(y),
       for Real, Var, |x: Real, y: Real| x.powf(y),
       for Var, Var, |x: Real, y: Real| x.powf(y),
       chain Fn = chain_pow);

#[cfg(test)]
mod test {
    use super::*;
    use core::memory::*;

    #[test]
    fn add() {
        let x: Real = 3.6;
        let y: Real = 3.0;        
        let stack = Rc::new(RefCell::new(ChainStack::new()));
        let vx = var!(stack, x);
        let vy = var!(stack, y);
        let v = pow(&vx, &vy);
        v.grad();
        assert_eq!(v.val(), x.clone().powf(y.clone()));
        assert_eq!(vx.adj(), y * x.powf(y - 1.0));
        assert_eq!(vy.adj(), x.powf(y) * x.ln());
    }
}
