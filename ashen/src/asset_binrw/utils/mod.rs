macro_rules! mod_use {
    ($($mod:ident),* $(,)?) => {
        $(
        mod $mod;
        #[allow(unused)]
        pub use $mod::*;
        )*
    };
}

mod_use!(
    color,
    normalized_f32,
    padded_null_string,
    pos_marker,
    texture,
    vec,
);

#[rustfmt::skip]
#[allow(unused_imports)]
pub use ::binrw::{*, helpers::*};
