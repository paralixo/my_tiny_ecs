pub mod world;
pub mod entities;
pub mod system;
pub mod query;
pub mod resources;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
