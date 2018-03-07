use core::vari::*;
use std::rc::Rc;
use core::memory::*;

pub fn grad(ptr: Rc<*mut Vari>) {
    let vi: &mut Vari = ptr.into();
    vi.init_dependent();
    let mem = vi.mem();
    mem.borrow().chain();
}
