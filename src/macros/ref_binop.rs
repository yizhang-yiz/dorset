
macro_rules! ref_binop {
    ( impl Vari for $varitype:ident,
      impl $imp:ident, $method:ident for Var, Real,
      with val = $value:expr,
      derivative = $derivative:expr) => {
        // {

        vari_type!($varitype, Real);

        impl Vari for $varitype {
            fn data(&self) -> &VariData {
                &self.data
            }
            fn data_mut(&mut self) -> &mut VariData {
                &mut self.data
            }
            fn chain(&mut self) {
                let adj = self.adj();
                let avi_adj = self.avi_.borrow().adj().clone();
                let avi_val = self.avi_.borrow().val().clone();
                let new_adj = avi_adj + $derivative(adj, avi_adj, avi_val, self.bd_);
                self.avi_.borrow_mut().set_adj(new_adj);
            }
        }

        impl<'a> $imp<Real> for &'a Var {
            type Output = Var;
            fn $method(self, other: Real) -> Var {
                let vi =
                    $varitype::new($value(self.val(), other),
                                   self.vi(), other);
                Var::new(self.storage(), Rc::new(RefCell::new(vi)))
            }
        }

        // }
    };

    ( impl Vari for $varitype:ident,
      impl $imp:ident, $method:ident for Real, Var,
      with val = $value:expr,
      derivative = $derivative:expr) => {
        // {

        vari_type!($varitype, Real);

        impl Vari for $varitype {
            fn data(&self) -> &VariData {
                &self.data
            }
            fn data_mut(&mut self) -> &mut VariData {
                &mut self.data
            }
            fn chain(&mut self) {
                let adj = self.adj();
                let avi_adj = self.avi_.borrow().adj().clone();
                let avi_val = self.avi_.borrow().val().clone();
                let new_adj = avi_adj + $derivative(adj, avi_adj, avi_val, self.bd_);
                self.avi_.borrow_mut().set_adj(new_adj);
            }
        }

        impl<'a> $imp<&'a Var> for Real {
            type Output = Var;
            fn $method(self, other: &Var) -> Var {
                let vi = $varitype::new($value(self, other.val()), other.vi(), self);
                Var::new(other.storage(), Rc::new(RefCell::new(vi)))
            }
        }

        // }
    };

    ( impl Vari for $varitype:ident,
      impl $imp:ident, $method:ident for Var, Var,
      with val = $value:expr,
      derivative a = $derivative_a:expr,
      derivative b = $derivative_b:expr) => {
        // {

        vari_type!($varitype, Vari);

        impl Vari for $varitype {
            fn data(&self) -> &VariData {
                &self.data
            }
            fn data_mut(&mut self) -> &mut VariData {
                &mut self.data
            }
            fn chain(&mut self) {
                let adj = self.adj();
                let avi_adj = self.avi_.borrow().adj().clone();
                let avi_val = self.avi_.borrow().val().clone();
                let bvi_adj = self.bvi_.borrow().adj().clone();
                let bvi_val = self.bvi_.borrow().val().clone();
                let new_avi_adj = avi_adj + $derivative_a(adj, avi_adj, avi_val, bvi_adj, bvi_val);
                let new_bvi_adj = bvi_adj + $derivative_b(adj, avi_adj, avi_val, bvi_adj, bvi_val);
                self.avi_.borrow_mut().set_adj(new_avi_adj);
                self.bvi_.borrow_mut().set_adj(new_bvi_adj);
            }
        }

        impl<'a, 'b> $imp<&'a Var> for &'b Var {
            type Output = Var;
            fn $method(self, other: &Var) -> Var {
                let vi = $varitype::new($value(self.val(), other.val()), self.vi(), other.vi());
                Var::new(self.storage(), Rc::new(RefCell::new(vi)))
            }
        }

        // }
    };
}
