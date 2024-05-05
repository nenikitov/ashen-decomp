use std::mem::{self, MaybeUninit};

// Code from [this post](https://www.reddit.com/r/learnrust/comments/lfw6uy/comment/gn16m4o)
pub trait CollectArray: Sized + Iterator {
    fn collect_array<const N: usize>(self) -> [Self::Item; N] {
        assert!(N > 0 && mem::size_of::<Self::Item>() > 0);
        let mut array = MaybeUninit::uninit();
        let array_ptr = array.as_mut_ptr() as *mut Self::Item;

        let mut i = 0;
        unsafe {
            for item in self {
                assert!(i < N);
                array_ptr.add(i).write(item);
                i += 1;
            }
            assert!(i == N);
            array.assume_init()
        }
    }
}

impl<T> CollectArray for T where T: Iterator {}
