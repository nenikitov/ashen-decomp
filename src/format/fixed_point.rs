use paste::paste;
use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops,
};

macro_rules! impl_operator {
    ($name: ident, $op: tt) => {
        paste! {
            impl<const P: u8> ops::[<$name:camel>] for Fixed<P> {
                type Output = Self;
                fn $name(self, rhs: Self) -> Self::Output {
                    let lhs: f32 = self.into();
                    let rhs: f32 = rhs.into();
                    (lhs $op rhs).into()
                }
            }

            impl<const P: u8> ops::[<$name:camel Assign>] for Fixed<P> {
                fn [<$name _assign>](&mut self, rhs: Self) {
                    let _ = std::mem::replace(self, (*self) $op rhs);
                }
            }
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Fixed<const PRECISION: u8> {
    value: i32,
}

impl<const P: u8> Debug for Fixed<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Fixed")
            .field("precision", &P)
            .field("internal", &self.value)
            .field("float", &Into::<f32>::into(*self))
            .finish()
    }
}

impl<const P: u8> Display for Fixed<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", Into::<f32>::into(*self))
    }
}

impl<const P: u8> Into<f32> for Fixed<P> {
    fn into(self) -> f32 {
        (self.value as f32) / 2f32.powi(P as i32)
    }
}

impl<const P: u8> From<f32> for Fixed<P> {
    fn from(value: f32) -> Self {
        Self {
            value: (value * 2f32.powi(P as i32)) as i32,
        }
    }
}

impl_operator!(add, +);
impl_operator!(sub, -);
impl_operator!(mul, *);
impl_operator!(div, /);

impl<const P: u8> ops::Neg for Fixed<P> {
    type Output = Fixed<P>;

    fn neg(self) -> Self::Output {
        return self * (-1f32).into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO Improve these tests

    #[test]
    fn fixed_point_works() {
        let f = Fixed::<16> { value: 0x28000 };
        let f: f32 = f.into();
        assert_eq!(f, 2.5);
    }

    #[test]
    fn fixed_point_works_2() {
        let f = Fixed::<16>::from(-2.5);
        assert_eq!(f.value, 0x28000);
    }
}
