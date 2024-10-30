use std::mem::MaybeUninit;

use itertools::Itertools;

pub trait IterMore: Iterator + Sized {
    // Adaptation from [this post](https://www.reddit.com/r/learnrust/comments/lfw6uy/comment/gn16m4o)
    fn collect_array<const N: usize>(self) -> [Self::Item; N] {
        // TODO(nenikitov): Replace with compile-time assertions or const generic expressions
        // When it will be supported.
        assert!(N > 0 && size_of::<Self::Item>() > 0);

        let mut array = MaybeUninit::<[Self::Item; N]>::uninit().transpose();

        Itertools::zip_eq(array.iter_mut(), self).for_each(|(dest, item)| _ = dest.write(item));

        // SAFETY: Every single element in the array is initialized, because we
        // wrote a valid iterator item into it.
        unsafe { array.transpose().assume_init() }
    }

    fn into_box_dyn(self) -> Box<dyn Iterator<Item = Self::Item>>
    where
        Self: 'static,
    {
        Box::new(self)
    }
}

impl<T> IterMore for T where T: Iterator {}
