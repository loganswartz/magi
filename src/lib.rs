pub mod cpu;
pub mod gb;
pub mod mmu;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
