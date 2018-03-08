use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;

fn chain_ln(vi: &Vari) {
    let adj = vi.adj();
    if let Operand::Vari(ref a) = vi.a {
        let avi: &mut Vari = a.clone().into();
        let avi_val = avi.val();
        let avi_adj = avi.adj();
        avi.set_adj(avi_adj + adj/avi_val);
    }
}

pub trait OpLn {
    fn ln(&self) -> Self;
}

impl OpLn for Real {
    fn ln(&self) -> Real {
        self.clone().ln()
    }
}

impl OpLn for Var {
    fn ln(&self) -> Var {
        let vi = self.get_vari_refmut();
        let mem = vi.mem();
        let operand = Operand::Vari(self.vi_.clone());
        let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
            self.val().ln(),
            operand,
            Operand::None,
            Box::new(chain_ln),
            mem.clone()
        ));
        Var::new(new_vi_ptr)
    }
}

pub fn ln<T: OpLn>(v: &T) -> T {
    OpLn::ln(v)
}

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
