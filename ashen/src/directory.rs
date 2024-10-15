// use crate::asset::{extension::Wildcard, AssetParser};
// use std::{io::Result as IoResult, path::Path};
//
// pub trait Directory {
//     fn get<A, P>(&self, path: P) -> IoResult<A>
//     where
//         A: AssetParser<Wildcard>,
//         P: AsRef<Path>;
//
//     fn all<A>(&self) -> IoResult<Vec<A>>
//     where
//         A: AssetParser<Wildcard> + 'static;
// }
