// #[macro_use]
// use macros::*;

// use operations::operand::Operand;
// use core::types::*;
// use core::constants::*;
// use core::chainable::*;
// use core::vari::*;
// use core::var::*;
// use std::cell::RefCell;
// use std::rc::Rc;
// use core::memory::*;

// macro_rules! vari_impl {
//     ( $varitype:ident,
//       with val = $vmethod:ident,
//       derivative = $derivative:expr) => {
//         // {

//         impl Vari for $varitype {
//             fn data(&self) -> &VariData {
//                 &self.data
//             }
//             fn data_mut(&mut self) -> &mut VariData {
//                 &mut self.data
//             }
//             fn chain(&mut self) {
//                 let adj = self.adj();
//                 let vi_adj = self.avi_.borrow().adj().clone();
//                 let vi_val = self.avi_.borrow().val().clone();
//                 let new_adj = vi_adj + $derivative(adj, vi_adj, vi_val);
//                 self.avi_.borrow_mut().set_adj(new_adj);
//             }
//         }

//         // }
//     };
// }
