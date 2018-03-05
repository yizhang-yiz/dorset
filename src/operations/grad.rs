use core::vari::*;
use std::rc::Rc;
use core::memory::*;

pub fn grad(refvi: Rc<*mut Vari>) {
    let vi: &mut Vari = refvi.into();
    vi.init_dependent();
    let mem = vi.mem();
    mem.borrow().chain();
}
