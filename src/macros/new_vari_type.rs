use std::fmt::Debug;
use core::types::*;
use core::constants::*;
use core::chainable::*;
use core::vari::*;

macro_rules! new_vari_type {
    ( $vari_type_name:ident ) => {
        // {
            #[derive(Debug)]
            struct $vari_type_name {
                val_: Real,
                adj_: Real,
            }

            impl $vari_type_name {
                fn new(v: Real) -> $vari_type_name {
                    $vari_type_name {
                        val_: v,
                        adj_: ZERO,
                    }
                }

                fn val(&self) -> Real {
                    self.val_.clone()
                }
                fn adj(&self) -> Real {
                    self.adj_.clone()
                }
                fn set_adj(&mut self, v: Real) {
                    self.adj_ = v;
                }    
            }

            impl Chainable for $vari_type_name {
                
            }
        // }
    };
}
