use std::ops::Div;

#[derive(Clone, Copy)]
pub struct TextureSize {
    pub width: usize,
    pub height: usize,
}

impl Div<usize> for TextureSize {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        TextureSize {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}
