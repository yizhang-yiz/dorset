macro_rules! binop {
    ( impl $trait:ident, $method:ident
      for Var, Real, $val1:expr,
      for Real, Var, $val2:expr,
      for Var, Var, $val3:expr,
      chain Fn = $chain_fn:expr ) => {
        // {
        impl<'a, 'b> $trait<&'a Real> for &'b Var {
            type Output = Var;
            fn $method(self, other: &'a Real) -> Var {
                let vi = self.get_vari_refmut();
                let mem = vi.mem();
                let opa = Operand::Vari(self.vi_.clone());
                let opb = Operand::Data(other.clone());
                let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
                    $val1(self.val(), other),
                    opa,
                    opb,
                    Box::new($chain_fn),
                    mem.clone()
                ));
                Var::new(new_vi_ptr)
            }
        }

        impl<'a, 'b> $trait<&'a Var> for &'b Real {
            type Output = Var;
            fn $method(self, other: &'a Var) -> Var {
                let vi = other.get_vari_refmut();
                let mem = vi.mem();
                let opa = Operand::Data(self.clone());
                let opb = Operand::Vari(other.vi_.clone());
                let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
                    $val2(self, other.val()),
                    opa,
                    opb,
                    Box::new($chain_fn),
                    mem.clone()
                ));
                Var::new(new_vi_ptr)
            }
        }

        impl<'a, 'b> $trait<&'a Var> for &'b Var {
            type Output = Var;
            fn $method(self, other: &'a Var) -> Var {
                let vi = self.get_vari_refmut();
                let mem = vi.mem();
                let opa = Operand::Vari(self.vi_.clone());
                let opb = Operand::Vari(other.vi_.clone());
                let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
                    $val3(self.val(), other.val()),
                    opa,
                    opb,
                    Box::new($chain_fn),
                    mem.clone()
                ));
                Var::new(new_vi_ptr)
            }
        }

        // }
    };

}
