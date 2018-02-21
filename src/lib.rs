#[macro_use]
pub mod macros;

mod core;
mod operations;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
