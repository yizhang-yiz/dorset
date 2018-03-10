//! Operation `ln`.
use super::*;

fn chain_ln(vi: &Vari) {
    let adj = vi.adj();
    if let Operand::Vari(ref a) = vi.a {
        let avi: &mut Vari = a.clone().into();
        let avi_val = avi.val();
        let avi_adj = avi.adj();
        avi.set_adj(avi_adj + adj / avi_val);
    }
}

uniop!{ impl OpLn, ln for Var, ln for Real, chain Fn = chain_ln }

#[cfg(test)]
mod test {
    use super::*;
    use core::memory::*;
    use float_cmp::*;

    #[test]
    fn test() {
        use std::mem::size_of;
        use std::cell::RefMut;
        use std::ops::Deref;
        let s = cstack!();
        let b: Var = var!(s, 3.0);
        let c = ln(&ln(&b));
        c.grad();
        let d: Real = b.val();
        let dloglogx = 1.0 / (d.clone().ln() * d);
        assert!(b.adj().approx_eq_ulps(&dloglogx, 2));
    }
}
