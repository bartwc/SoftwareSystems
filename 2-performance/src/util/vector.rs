use crate::util::color::Color;
use crate::util::get_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f32;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

const EPSILON: f32 = 0.00001;

trait Clamp01 {
    fn clamp01(self) -> Self;
}

impl Clamp01 for f32 {
    fn clamp01(self) -> Self {
        self.min(1.).max(0.)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Makes a vector from one value, making the x, y and z component the same
    pub fn repeated(a: f32) -> Self {
        Self { x: a, y: a, z: a }
    }

    pub fn from_arr([a, b, c]: [f32; 3]) -> Self {
        Self::new(a as f32, b as f32, c as f32)
    }

    pub fn iszero(&self) -> bool {
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length2(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length2().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        if length > 0f32 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }
    }

    pub fn unit(&self) -> Vector {
        let length = self.length();
        Vector::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn powf(&self, exp: f32) -> Vector {
        Vector::new(self.x.powf(exp), self.y.powf(exp), self.z.powf(exp))
    }

    pub fn max_item(&self) -> f32 {
        if self.x > self.y {
            if self.x > self.z {
                return self.x;
            }
            self.z
        } else {
            if self.y > self.z {
                return self.y;
            }
            self.z
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn gamma(&self, exp: f32) -> Vector {
        self.powf(exp) * (exp + 1f32) / 2f32
    }

    pub fn rotated(&self, rotation: Vector) -> Vector {
        let nt = if rotation.x.abs() > rotation.y.abs() {
            Vector::new(rotation.z, 0f32, -rotation.x)
                / (rotation.x.powi(2) + rotation.z.powi(2)).sqrt()
        } else {
            Vector::new(0f32, -rotation.z, rotation.y)
                / (rotation.y.powi(2) + rotation.z.powi(2)).sqrt()
        };

        let nb = rotation.cross(nt);

        let x = self.x * nb.x + self.y * rotation.x + self.z * nt.x;
        let y = self.x * nb.y + self.y * rotation.y + self.z * nt.y;
        let z = self.x * nb.z + self.y * rotation.z + self.z * nt.z;

        Vector::new(x, y, z)
    }

    pub fn point_on_hemisphere() -> Vector {
        let theta = get_rng().gen::<f32>() * 2f32 * f32::consts::PI;
        let phi = (1f32 - 2f32 * get_rng().gen::<f32>()).acos();

        Vector::new(
            phi.sin() * theta.cos(),
            (phi.sin() * theta.sin()).abs(),
            phi.cos(),
        )
    }

    pub fn point_on_sphere() -> Vector {
        let theta = get_rng().gen::<f32>() * 2f32 * f32::consts::PI;
        let phi = (1f32 - 2f32 * get_rng().gen::<f32>()).acos();

        Vector::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos())
    }

    pub fn point_on_diffuse_hemisphere() -> Vector {
        let u = get_rng().gen::<f32>();
        let v = 2. * f32::consts::PI * get_rng().gen::<f32>();

        Vector::new(v.cos() * u.sqrt(), (1. - u).sqrt(), v.sin() * u.sqrt())
    }
}

impl Into<Color> for Vector {
    fn into(self) -> Color {
        Color {
            r: (self.x.clamp01() * 255.) as u8,
            g: (self.y.clamp01() * 255.) as u8,
            b: (self.z.clamp01() * 255.) as u8,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::color::Color;
    use crate::util::vector::Vector;

    #[test]
    fn test_add() {
        let a = Vector::new(1f32, 2f32, 3f32);
        let b = Vector::new(5f32, 3f32, 2f32);

        let c = a + b;

        assert_eq!(c, Vector::new(6f32, 5f32, 5f32));
    }

    #[test]
    fn test_to_color_1() {
        let a: Vector = Vector::new(5., -5., 0.5);
        let c: Color = a.into();

        assert_eq!(
            c,
            Color {
                r: 255,
                g: 0,
                b: 127
            }
        );
    }
}
