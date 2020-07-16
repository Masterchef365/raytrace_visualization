use nalgebra::{Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn reflect(&self, intersection: &Point3<f32>, normal: &Vector3<f32>) -> Self {
        Ray {
            origin: *intersection,
            direction: self.direction - 2.0 * normal * normal.dot(&self.direction)
        }
    }
}
