use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct TextureCoordinate {
    pub u: f32,
    pub v: f32,
}

impl TextureCoordinate {
    pub fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }
}

impl Add for TextureCoordinate {
    type Output = TextureCoordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u + rhs.u,
            v: self.v + rhs.v,
        }
    }
}

impl Sub for TextureCoordinate {
    type Output = TextureCoordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        }
    }
}

impl Sub for &TextureCoordinate {
    type Output = TextureCoordinate;

    fn sub(self, rhs: &TextureCoordinate) -> Self::Output {
        TextureCoordinate {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        }
    }
}

impl Mul for TextureCoordinate {
    type Output = TextureCoordinate;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u * rhs.u,
            v: self.v * rhs.v,
        }
    }
}

impl Mul<f32> for TextureCoordinate {
    type Output = TextureCoordinate;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            u: self.u * rhs,
            v: self.v * rhs,
        }
    }
}
