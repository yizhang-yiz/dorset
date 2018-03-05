use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;

fn chain_ln(vi: &Vari) {
    let adj = vi.adj();
    if let Operand::Vari(ref rcavi) = vi.a {
        let avi: &mut Vari = rcavi.clone().into();
        let avi_val = avi.val();
        let avi_adj = avi.adj();
        avi.set_adj(avi_adj + adj/avi_val);
    }
}

pub fn ln(v: &Var) -> Var {
    let vi: &mut Vari = v.vi_.clone().into();
    let mem = vi.mem();
    let operand = Operand::Vari(v.vi_.clone());
    let new_vi = mem.borrow_mut().alloc(Vari::new(
        v.val().ln(),
        operand,
        Operand::None,
        Box::new(chain_ln),
        mem.clone()
    ));
    Var::new(new_vi)
}

// pub fn ln(v: &Var) -> Var{
//     v.ln()
// }

#[cfg(test)]
mod test {
    use super::*;
    use core::memory::*;
    use float_cmp::*;

    #[test]
    fn test_vari0() {
        use std::mem::size_of;
        use std::cell::RefMut;
        use std::ops::Deref;
        let s = Rc::new(RefCell::new(ChainStack::new()));
        let a: Var = var!(s);
        let b: Var = var!(s, 3.0);
        let c = ln(&ln(&b));
        c.grad();
        let d: Real = b.val();
        let dloglogx = 1.0/(d.clone().ln() * d);
        assert!(b.adj().approx_eq_ulps(&dloglogx,2));
    }
}
