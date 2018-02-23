
macro_rules! var_uniop {
    ( impl Vari for $varitype:ident,
      $method:ident for Var,
      with val = $vmethod:ident,
      derivative = $derivative:expr) => {
        // {

        vari_type!($varitype);

        impl Vari for $varitype {
            fn data(&self) -> &VariData {
                &self.data
            }
            fn data_mut(&mut self) -> &mut VariData {
                &mut self.data
            }
            fn chain(&mut self) {
                let adj = self.adj();
                let vi_adj = self.avi_.borrow().adj().clone();
                let vi_val = self.avi_.borrow().val().clone();
                let new_adj = vi_adj + $derivative(adj, vi_adj, vi_val);
                self.avi_.borrow_mut().set_adj(new_adj);
            }
        }

        impl Var {
            fn $method(&self) -> Var {
                let vi = $varitype::new(self.val().$vmethod(), self.vi());
                Var::new(self.storage(), Rc::new(RefCell::new(vi)))
            }
        }

        // }
    };
}
