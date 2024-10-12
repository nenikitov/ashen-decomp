use itertools::Itertools;

// Code from [this post](https://www.reddit.com/r/learnrust/comments/lfw6uy/comment/gn16m4o)
pub trait CollectArray: Sized + Iterator {
    fn collect_array<const N: usize>(self) -> [Self::Item; N] {
        // TODO(nenikitov): Replace with compile-time assertions or const generic expressions
        // When it will be supported.
        assert!(N > 0 && size_of::<Self::Item>() > 0);

        let mut array = std::mem::MaybeUninit::<[Self::Item; N]>::uninit().transpose();

        Itertools::zip_eq(array.iter_mut(), self).for_each(|(dest, item)| _ = dest.write(item));

        // SAFETY: Every single element in the array is initialized
        // because we wrote a valid iterator element into it.
        unsafe { array.transpose().assume_init() }
    }
}

impl<T> CollectArray for T where T: Iterator {}
