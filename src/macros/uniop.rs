macro_rules! uniop {
    ( impl $trait:ident, $method:ident for Var, $dmethod:ident for Real,
      chain Fn = $chain_fn:expr ) => {
        // {

        pub trait $trait {
            fn $method(&self) -> Self;
        }

        impl $trait for Real {
            fn $method(&self) -> Real {
                self.clone().$dmethod()
            }
        }

        impl $trait for Var {
            fn $method(&self) -> Var {
                let vi = self.get_vari_refmut();
                let mem = vi.mem();
                let operand = Operand::Vari(self.vi_.clone());
                let new_vi_ptr = mem.borrow_mut().alloc(Vari::new(
                    self.val().$dmethod(),
                    operand,
                    Operand::None,
                    Box::new($chain_fn),
                    mem.clone()
                ));
                Var::new(new_vi_ptr)
            }
        }

        pub fn $method<T: $trait>(v: &T) -> T {
            $trait::$method(v)
        }

        // }
    };

}
