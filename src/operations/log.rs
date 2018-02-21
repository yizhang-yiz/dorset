use core::types::*;
use core::constants::*;
use core::chainable::*;
use core::vari::*;

#[macro_use]
use macros;

new_vari_type!(LogVari);

#[cfg(test)]
// mod log_test {
    #[test]
    fn it_works() {
        let x = LogVari::new(3.0);
        let mut var_stack = Vec::new();
        let mut s = ChainableStackStorage::new(var_stack);
        s.push(x);
    }
// }
