macro_rules! mod_use {
    ($($mod:ident),* $(,)?) => {
        $(
            mod $mod;
            #[allow(unused_imports)]
            pub use $mod::*;
        )*
    };
}

mod_use!(
    color,
    entire_file,
    marker,
    normalized_f32,
    padded_null_string,
    texture,
    vec,
    zlib,
);

#[rustfmt::skip]
#[allow(unused_imports)]
pub use ::binrw::{*, helpers::*};
