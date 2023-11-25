use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
// use std::io::{stdout, Write};
use std::sync::{Arc};

use crate::util::vector::Vector;

#[derive(Debug)]
pub struct MSTracer {
    samples_per_pixel: usize,
}

impl MSTracer {
    pub fn new(samples_per_pixel: usize) -> Self {
        Self { samples_per_pixel }
    }
}

impl RayTracer for MSTracer {
    fn raytrace<'a>(
        &self,
        x: usize,
        y: usize,
        datastructure: Arc<dyn DataStructure>,
        shader: &'a dyn Shader,
        camera: &Camera,
    ) -> Vector {
        let mut out = Vector::repeated(0f32);
        let ray = camera.generate_ray(x as f32, y as f32);
        // let ray_clone = ray.clone();
        let intersection = datastructure.intersects(&ray);
        for _ in 0..self.samples_per_pixel {
            out += shader.shade(&ray, datastructure.clone(), &intersection)/ self.samples_per_pixel as f32 ;
        }
        print!("\r{x}, {y} ");
        // stdout().flush().unwrap();

        out
    }
}
