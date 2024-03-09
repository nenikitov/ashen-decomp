use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FineTune {
    cents: i32,
}

impl FineTune {
    const CENTS_PER_NOTE: i32 = 128;

    pub const fn new(cents: i32) -> Self {
        Self { cents }
    }

    pub const fn from_note(note: i32) -> Self {
        FineTune::new(note * Self::CENTS_PER_NOTE)
    }

    pub fn pitch_factor(self) -> f64 {
        // TODO(nenikitov): This formula is from the game
        // And it's very magic.
        // Maybe simplify it or at least name constants.
        1.0 / (2f64.powf((self.cents as f64 + 1.0) / (12.0 * FineTune::CENTS_PER_NOTE as f64))
            * 8363.0
            // TODO(nenikitov): This is `2^20`, which is divided by `2048` and `8192` results in `1/16`
            * 1048576.0
            / 16000.0
            / 2048.0
            / 8192.0)
    }

    pub fn cents(self) -> i32 {
        self.cents
    }

    pub fn note(self) -> i32 {
        self.cents / Self::CENTS_PER_NOTE
    }
}

impl Add for FineTune {
    type Output = FineTune;

    fn add(self, rhs: Self) -> Self::Output {
        FineTune::new(self.cents.saturating_add(rhs.cents))
    }
}

impl Sub for FineTune {
    type Output = FineTune;

    fn sub(self, rhs: Self) -> Self::Output {
        FineTune::new(self.cents.saturating_sub(rhs.cents))
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn from_note_works() {
        assert_eq!(
            FineTune {
                cents: 30 * FineTune::CENTS_PER_NOTE
            },
            FineTune::from_note(30)
        );
    }

    #[test]
    fn pitch_factor_works() {
        assert_approx_eq!(2.0, FineTune::from_note(47).pitch_factor(), 0.030);
        assert_approx_eq!(1.0, FineTune::from_note(59).pitch_factor(), 0.015);
        assert_approx_eq!(0.5, FineTune::from_note(71).pitch_factor(), 0.008);
    }

    #[test]
    fn add_works() {
        assert_eq!(
            FineTune::from_note(54),
            FineTune::from_note(49) + FineTune::from_note(5),
        );
    }

    #[test]
    fn sub_works() {
        assert_eq!(
            FineTune::from_note(32),
            FineTune::from_note(40) - FineTune::from_note(8),
        );
    }
}
