use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use std::f32;
use std::sync::Arc;

pub enum Axis {
    X(f32),
    Y(f32),
    Z(f32),
}

impl Axis {
    pub fn divide(
        &self,
        bounding_box: &BoundingBox,
        steps: usize,
    ) -> Vec<(BoundingBox, BoundingBox)> {
        match self {
            Axis::X(length) => (0..steps)
                .map(|i| bounding_box.split_at(Axis::X((1. / steps as f32 * length) * i as f32)))
                .collect(),
            Axis::Y(length) => (0..steps)
                .map(|i| bounding_box.split_at(Axis::Y((1. / steps as f32 * length) * i as f32)))
                .collect(),
            Axis::Z(length) => (0..steps)
                .map(|i| bounding_box.split_at(Axis::Z((1. / steps as f32 * length) * i as f32)))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub(super) min: Vector,
    pub(super) max: Vector,
}

impl BoundingBox {
    pub const EMPTY: BoundingBox = BoundingBox {
        min: Vector {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        },
        max: Vector {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        },
    };

    pub fn new(min: Vector, max: Vector) -> Self {
        Self { min, max }
    }

    pub fn from_triangle(triangle: Triangle) -> Self {
        Self::EMPTY
            .include_point(triangle.a())
            .include_point(triangle.b())
            .include_point(triangle.c())
    }

    pub fn merge(&self, other: &Self) -> Self {
        self.include_point(other.min).include_point(other.max)
    }

    pub fn include_point(&self, point: Vector) -> Self {
        Self {
            min: self.min.min(&(point - Vector::repeated(0.01))),
            max: self.max.max(&(point + Vector::repeated(0.01))),
        }
    }

    pub fn from_triangles(triangles: impl Iterator<Item = Triangle>) -> Self {
        let mut curr = Self::EMPTY;
        for i in triangles {
            curr = curr.merge(&BoundingBox::from_triangle(i));
        }

        curr
    }

    pub fn size(&self) -> Vector {
        let x = self.max.x - self.min.x;
        let y = self.max.y - self.min.y;
        let z = self.max.z - self.min.z;

        Vector::new(x, y, z)
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.size();
        let surface_top = size.x * size.z;
        let surface_front = size.x * size.y;
        let surface_side = size.y * size.z;

        2. * (surface_top + surface_front + surface_side)
    }

    pub fn cost(&self, numtriangles: usize) -> f32 {
        self.surface_area() * numtriangles as f32
    }

    pub fn contains(&self, triangle: &Triangle) -> bool {
        if triangle.a().x < self.min.x && triangle.b().x < self.min.x && triangle.c().x < self.min.x
        {
            return false;
        }
        if triangle.a().y < self.min.y && triangle.b().y < self.min.y && triangle.c().y < self.min.y
        {
            return false;
        }
        if triangle.a().z < self.min.z && triangle.b().z < self.min.z && triangle.c().z < self.min.z
        {
            return false;
        }

        if triangle.a().x > self.max.x && triangle.b().x > self.max.x && triangle.c().x > self.max.x
        {
            return false;
        }
        if triangle.a().y > self.max.y && triangle.b().y > self.max.y && triangle.c().y > self.max.y
        {
            return false;
        }
        if triangle.a().z > self.max.z && triangle.b().z > self.max.z && triangle.c().z > self.max.z
        {
            return false;
        }
        true
    }

    pub fn split_at(&self, axis: Axis) -> (BoundingBox, BoundingBox) {
        match axis {
            Axis::X(i) => (
                BoundingBox {
                    min: self.min,
                    max: Vector::new(self.min.x + i, self.max.y, self.max.z),
                },
                BoundingBox {
                    min: Vector::new(self.min.x + i, self.min.y, self.min.z),
                    max: self.max,
                },
            ),
            Axis::Y(i) => (
                BoundingBox {
                    min: self.min,
                    max: Vector::new(self.max.x, self.min.y + i, self.max.z),
                },
                BoundingBox {
                    min: Vector::new(self.min.x, self.min.y + i, self.min.z),
                    max: self.max,
                },
            ),
            Axis::Z(i) => (
                BoundingBox {
                    min: self.min,
                    max: Vector::new(self.max.x, self.max.y, self.min.z + i),
                },
                BoundingBox {
                    min: Vector::new(self.min.x, self.min.y, self.min.z + i),
                    max: self.max,
                },
            ),
        }
    }

    pub fn longest_axis(&self) -> Axis {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;

        if dx > dy && dx > dz {
            Axis::X(dx)
        } else if dx > dy && dx <= dz {
            Axis::Z(dz)
        } else if dx <= dy && dy > dz {
            Axis::Y(dy)
        } else {
            Axis::Z(dz)
        }
    }

    pub fn includes_point(&self, point: &Vector) -> bool {
        if point.x < self.min.x {
            return false;
        }
        if point.y < self.min.y {
            return false;
        }
        if point.z < self.min.z {
            return false;
        }

        if point.x > self.max.x {
            return false;
        }
        if point.y > self.max.y {
            return false;
        }
        if point.z > self.max.z {
            return false;
        }

        true
    }
}

#[cfg(test)]
pub mod tests {
    use crate::datastructure::bvh::boundingbox::BoundingBox;
    use crate::util::vector::Vector;

    #[test]
    fn test_create() {
        let bb = BoundingBox::new(Vector::new(0., 0., 0.), Vector::new(1., 1., 1.));
        assert_eq!(bb.min, Vector::new(0., 0., 0.));
        assert_eq!(bb.max, Vector::new(1., 1., 1.));
    }
}
