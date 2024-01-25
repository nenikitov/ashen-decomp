use std::ops::Div;

pub struct TextureSize {
    pub width: u16,
    pub height: u16,
}

impl Div<u16> for TextureSize {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        &self / rhs
    }
}

impl Div<u16> for &TextureSize {
    type Output = TextureSize;

    fn div(self, rhs: u16) -> Self::Output {
        TextureSize {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}
