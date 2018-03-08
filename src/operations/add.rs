use core::types::*;
use core::vari::*;
use core::var::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Add;

fn chain_add(vi: &Vari) {
    let adj = vi.adj();
    match (vi.a.clone(), vi.b.clone()) {
        (Operand::Vari(ptr), Operand::Data(bd)) => {
            let avi: &mut Vari = ptr.clone().into();
            let avi_val = avi.val();
            let avi_adj = avi.adj();
            avi.set_adj(avi_adj + adj);
        }
        (Operand::Data(ad), Operand::Vari(ptr)) => {
            let bvi: &mut Vari = ptr.clone().into();
            let bvi_val = bvi.val();
            let bvi_adj = bvi.adj();
            bvi.set_adj(bvi_adj + adj);
        }
        (Operand::Vari(ap), Operand::Vari(bp)) => {
            let avi: &mut Vari = ap.clone().into();
            let avi_val = avi.val();
            let avi_adj = avi.adj();
            avi.set_adj(avi_adj + adj);
            let bvi: &mut Vari = bp.clone().into();
            let bvi_val = bvi.val();
            let bvi_adj = bvi.adj();
            bvi.set_adj(bvi_adj + adj);
        }
        _ => {}
    }
}

impl<'a> Add<&'a Var> for &'a Var {
    type Output = Var;
    fn add(self, other: &'a Var) -> Var {
        let vi = self.get_vari_refmut();
        let mem = vi.mem();
        let opa = Operand::Vari(self.vi_.clone());
        let opb = Operand::Vari(other.vi_.clone());
        let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
            self.val() + other.val(),
            opa,
            opb,
            Box::new(chain_add),
            mem.clone()
        ));
        Var::new(new_vi_ptr)
    }
}

impl<'a> Add<&'a Real> for &'a Var {
    type Output = Var;
    fn add(self, other: &'a Real) -> Var {
        let vi = self.get_vari_refmut();
        let mem = vi.mem();
        let opa = Operand::Vari(self.vi_.clone());
        let opb = Operand::Data(other.clone());
        let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
            self.val() + other,
            opa,
            opb,
            Box::new(chain_add),
            mem.clone()
        ));
        Var::new(new_vi_ptr)
    }
}

impl<'a> Add<&'a Var> for &'a Real {
    type Output = Var;
    fn add(self, other: &'a Var) -> Var {
        let vi = other.get_vari_refmut();
        let mem = vi.mem();
        let opa = Operand::Vari(other.vi_.clone());
        let opb = Operand::Data(self.clone());
        let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
            other.val() + self,
            opa,
            opb,
            Box::new(chain_add),
            mem.clone()
        ));
        Var::new(new_vi_ptr)
    }
}

#[cfg(test)]
mod add_test {
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
