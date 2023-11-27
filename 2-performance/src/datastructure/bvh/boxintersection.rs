use crate::datastructure::bvh::boundingbox::BoundingBox;
use crate::util::ray::Ray;

#[derive(Debug)]
pub struct BoxIntersection {
    pub ray: Ray,
    pub t: f32,
    pub boundingbox: BoundingBox,
}
