use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::f32;

#[derive(Debug)]
pub struct Camera {
    pub pos: Vector,
    pub direction: Vector,
    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub inf_width: f32,
    pub inf_height: f32,
    pub angle: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(pos: Vector, direction: Vector, width: usize, height: usize, fov: f32) -> Self {
        let inf_width = 1f32 / (width as f32);
        let inf_height = 1f32 / (height as f32);
        let angle = (f32::consts::PI * 0.5f32 * fov / 180f32).tan();
        let aspect_ratio = width as f32 / height as f32;
        let internal_direction = direction.rotated(Vector::new(0., 0., 1.)).unit();

        Self {
            pos,
            direction: internal_direction,
            width,
            height,
            fov,
            inf_width,
            inf_height,
            angle,
            aspect_ratio,
        }
    }

    pub fn generate_ray(&self, x: f32, y: f32) -> Ray {
        let xdir = (2f32 * x as f32 * self.inf_width - 1f32) * self.angle * self.aspect_ratio;
        let ydir = (1f32 - 2f32 * y as f32 * self.inf_height) * self.angle;

        let raydir = Vector::new(xdir, ydir, -1f32)
            .rotated(Vector::new(0., 0., 1.))
            .rotated(self.direction)
            .rotated(Vector::new(0., 0., -1.));

        Ray::new(self.pos, raydir)
    }
}
