use std::ops::{Add, Div, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FineTune {
    cents: i32,
}

impl FineTune {
    const BASE_NOTE: FineTune = FineTune::new_from_note(49);
    const BASE_FREQUENCY: f64 = 440.0;

    const CENTS_PER_NOTE: i32 = 128;

    pub const fn new(cents: i32) -> Self {
        Self { cents }
    }

    pub const fn new_from_note(note: i32) -> Self {
        FineTune::new(note * Self::CENTS_PER_NOTE)
    }

    pub fn frequency(&self) -> f64 {
        Self::BASE_FREQUENCY
            * 2.0f64
                .powf((*self - Self::BASE_NOTE).cents as f64 / (12 * Self::CENTS_PER_NOTE) as f64)
    }

    pub fn cents(&self) -> i32 {
        self.cents
    }

    pub fn note(&self) -> i32 {
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

impl Div for FineTune {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.frequency() / rhs.frequency()
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn new_works() {
        assert_eq!(FineTune { cents: 1000 }, FineTune::new(1000));
    }

    #[test]
    fn new_from_note_works() {
        assert_eq!(
            FineTune {
                cents: 30 * FineTune::CENTS_PER_NOTE
            },
            FineTune::new_from_note(30)
        );
    }

    #[test]
    fn new_cent_to_note_relation_works() {
        assert_eq!(
            FineTune::new(FineTune::CENTS_PER_NOTE),
            FineTune::new_from_note(1)
        );
    }

    #[test]
    fn cents_works() {
        assert_eq!(100, FineTune::new(100).cents())
    }

    #[test]
    fn note_works() {
        assert_eq!(5, FineTune::new_from_note(5).note())
    }

    #[test]
    fn frequency_works() {
        assert_eq!(440.0, FineTune::BASE_NOTE.frequency());
        assert_eq!(
            880.0,
            (FineTune::BASE_NOTE + FineTune::new_from_note(12)).frequency()
        );
        assert_eq!(
            220.0,
            (FineTune::BASE_NOTE - FineTune::new_from_note(12)).frequency()
        );
        assert_approx_eq!(
            392.0,
            (FineTune::BASE_NOTE - FineTune::new_from_note(2)).frequency(),
            0.25
        );
    }

    #[test]
    fn add_works() {
        assert_eq!(
            FineTune::new_from_note(54),
            FineTune::new_from_note(49) + FineTune::new_from_note(5),
        );
    }

    #[test]
    fn sub_works() {
        assert_eq!(
            FineTune::new_from_note(32),
            FineTune::new_from_note(40) - FineTune::new_from_note(8),
        );
    }

    #[test]
    fn div_works() {
        assert_approx_eq!(
            2.0,
            FineTune::new_from_note(30) / FineTune::new_from_note(18),
            0.01
        );
        assert_approx_eq!(
            0.5,
            FineTune::new_from_note(18) / FineTune::new_from_note(30),
            0.01
        );
        assert_approx_eq!(
            1.5,
            FineTune::new_from_note(17) / FineTune::new_from_note(10),
            0.01
        );
    }
}
