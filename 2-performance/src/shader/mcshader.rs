use crate::datastructure::DataStructure;
use crate::shader::{diffuse, emittance, Shader};
use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::sync::{Arc};
use crate::datastructure::intersection::Intersection;

#[derive(Debug)]
pub struct McShader;

impl McShader {
    pub fn shade_internal<'a>(
        &self,
        _ray: &Ray,
        depth: usize,
        datastructure: Arc<dyn DataStructure>,
        intersection: &Option<Intersection>,
    ) -> Vector {
        if let Some(intersection_ref) = intersection {
            let hit_pos = intersection_ref.hit_pos();
            let part_emi = emittance(intersection_ref);
            let indirect = if depth > 0 {
                let bounce_direction =
                    Vector::point_on_hemisphere().rotated(intersection_ref.triangle.normal());
                let bounce_ray = Ray::new(hit_pos, bounce_direction);
                let datastructure_clone = datastructure.clone();
                let intersection_test = datastructure_clone.intersects(&bounce_ray);
                let indirect_light = self.shade_internal(&bounce_ray, depth - 1, datastructure_clone, &intersection_test);
                indirect_light * diffuse(intersection_ref, hit_pos, hit_pos + bounce_direction)
            } else {
                Vector::repeated(0f32)
            };

            return indirect * 2. + part_emi;
        }
        else {
            return Vector::repeated(0f32);
        }
    }
}

impl Shader for McShader {
    fn shade<'a> (&self, ray: &Ray, datastructure: Arc<dyn DataStructure>, intersection: &Option<Intersection>) -> Vector {
        self.shade_internal(ray, 4, datastructure.clone(), intersection)
    }
}
