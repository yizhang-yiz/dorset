//! Operation `cos`
use super::*;
pub use std::rc::Rc;

fn chain_cos(vi: &Vari) {
    let adj = vi.adj();
    if let Operand::Vari(ref a) = vi.a {
        let avi: &mut Vari = a.clone().into();
        let avi_val = avi.val();
        let avi_adj = avi.adj();
        avi.set_adj(avi_adj - adj * avi_val.sin());
    }
}

uniop!{ impl OpCos, cos for Var, cos for Real, chain Fn = chain_cos }

#[cfg(test)]
mod test {
    use super::*;
    use core::constants::*;
    use core::memory::*;
    use float_cmp::*;

    #[test]
    fn test() {
        use std::mem::size_of;
        use std::cell::RefMut;
        use std::ops::Deref;
        let s = Rc::new(RefCell::new(ChainStack::new()));
        const X: Real = 5.0;
        let a: Var = var!(s);
        let b: Var = var!(s, X.clone());
        let c = cos(&a);
        c.grad();
        assert!(a.adj().approx_eq_ulps(&ZERO, 2));
        c.set_zero_all_adjoints();
        let d = cos(&b);
        d.grad();
        assert!(b.adj().approx_eq_ulps(&-X.sin(), 2));
    }
}
