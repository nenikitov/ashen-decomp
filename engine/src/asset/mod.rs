pub mod gamma_table;

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    GammaTable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Extension {
    #[default]
    Dat,
    Custom(String),
}

pub trait Asset
where
    Self: Sized,
{
    fn kind() -> Kind;
    // TODO(Unavailable): Result > panic!
    fn parse(bytes: &[u8], extension: Extension) -> Self;
}
