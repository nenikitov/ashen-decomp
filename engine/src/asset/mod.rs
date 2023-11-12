pub mod gamma_table;

pub enum Kind {
    GammaTable,
}

pub trait Asset where Self: Sized {
    fn kind() -> Kind;
    // TODO(Unavailable): Result > panic!
    fn parse(bytes: &[u8], extension: &str) -> Self;
}
